import type { AstroIntegration, AstroIntegrationLogger } from 'astro'
import { execFile } from 'node:child_process'
import { readdir } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { promisify } from 'node:util'

/**
 * Astro integration that builds the workspace's wasm-bindgen crates, for
 * imports like `#rust/smoke`
 *
 * - No browser APIs in code that runs at build time
 * - Call imported functions directly, as hot updates replace the module
 * - Build-time and browser instances share no memory, so pass state through the HTML
 *
 */
export function rust(): AstroIntegration {
  return {
    name: 'rust',
    hooks: {
      'astro:config:setup': async ({ command, updateConfig, logger }) => {
        if (command === 'preview') return

        updateConfig({
          vite: {
            plugins: [
              {
                // Astro moves client assets before prerendering, breaking server-side
                // wasm URLs, so inline wasm everywhere but the client
                name: 'rust:inline-server-wasm',
                configEnvironment: (name) =>
                  name === 'client'
                    ? undefined
                    : {
                        build: { assetsInlineLimit: (file) => file.endsWith('.wasm') || undefined },
                      },
              },
            ],
          },
        })

        if (command === 'dev') await buildInDev(logger)
        else await build('wasm-release', logger)
      },

      'astro:server:setup': ({ server, logger }) => {
        let queued = false
        let latest = Promise.resolve()

        // Build after the current build, once, however many files change meanwhile
        function rebuild() {
          if (queued) return
          queued = true
          latest = latest.then(async () => {
            queued = false
            // Vite ignores target/, so announce the new output
            for (const file of await buildInDev(logger)) server.watcher.emit('change', file)
          })
        }

        server.watcher.on('all', (_event, file) => {
          if (/\.rs$|\/Cargo\.(toml|lock)$/.test(file)) rebuild()
        })
      },
    },
  }
}

type Profile = 'dev' | 'wasm-release'

interface Crate {
  name: string
  libName: string
}

interface Package {
  name: string
  dependencies: { name: string }[]
  targets: { name: string; kind: string[] }[]
}

const targetDir = fileURLToPath(new URL('../../target/', import.meta.url))

// Rust's default wasm32 target features, which wasm-opt can't detect
const wasmOptFeatures = [
  'bulk-memory',
  'multivalue',
  'mutable-globals',
  'nontrapping-float-to-int',
  'reference-types',
  'sign-ext',
].map((feature) => `--enable-${feature}`)

const exec = promisify(execFile)

async function findWasmCrates(): Promise<Crate[]> {
  const { stdout } = await exec('cargo', ['metadata', '--no-deps', '--format-version=1'])
  const { packages } = JSON.parse(stdout) as { packages: Package[] }
  return packages.flatMap(({ name, dependencies, targets }) => {
    const usesWasmBindgen = dependencies.some((d) => d.name === 'wasm-bindgen')
    const lib = targets.find(({ kind }) => kind.includes('cdylib'))
    return usesWasmBindgen && lib ? [{ name, libName: lib.name }] : []
  })
}

// Returns the files written
async function generateBindings({ name, libName }: Crate, profile: Profile): Promise<string[]> {
  // Cargo writes dev builds to debug/
  const wasm = `${targetDir}wasm32-unknown-unknown/${profile === 'dev' ? 'debug' : profile}/${libName}.wasm`
  const outDir = `${targetDir}wasm-bindgen/${name}`
  const wasmFile = `${outDir}/index_bg.wasm`

  await exec('wasm-bindgen', ['--target=bundler', '--out-name=index', `--out-dir=${outDir}`, wasm])
  if (profile === 'wasm-release') {
    await exec('wasm-opt', ['-Oz', ...wasmOptFeatures, wasmFile, '-o', wasmFile])
  }

  return (await readdir(outDir)).map((file) => `${outDir}/${file}`)
}

async function build(profile: Profile, logger: AstroIntegrationLogger): Promise<string[]> {
  const start = performance.now()
  const crates = await findWasmCrates()
  if (crates.length === 0) return []

  const packageArgs = crates.map(({ name }) => `--package=${name}`)
  await exec('cargo', [
    'build',
    '--quiet',
    '--target=wasm32-unknown-unknown',
    `--profile=${profile}`,
    ...packageArgs,
  ])
  const files = (await Promise.all(crates.map((crate) => generateBindings(crate, profile)))).flat()

  const names = crates.map(({ name }) => name).join(', ')
  logger.info(`Built ${names} (${profile}) in ${Math.round(performance.now() - start)} ms`)
  return files
}

// Broken Rust never stops the dev server, only builds and checks
async function buildInDev(logger: AstroIntegrationLogger): Promise<string[]> {
  try {
    return await build('dev', logger)
  } catch (error) {
    logger.error(String(error))
    return []
  }
}
