import type { AstroIntegration, AstroIntegrationLogger } from 'astro'
import { execFile } from 'node:child_process'
import { readdir } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { promisify } from 'node:util'

/**
 * Astro integration for importing Rust as WebAssembly
 *
 * Importing a crate looks like `#rust/example`.
 *
 * - Rust libraries can be called at build time or run time
 * - Basic hot updates
 * - Crates are compiled using wasm-bindgen
 *
 */
export function rust(): AstroIntegration {
  return {
    name: 'rust',
    hooks: {
      /**
       * Build before anything imports the bindings
       *
       * This hook runs before Vite starts, for `dev`, `build`, and `sync`.
       *
       */
      'astro:config:setup': async ({ command, updateConfig, logger }) => {
        if (command === 'preview') return

        updateConfig({
          vite: {
            plugins: [
              {
                // Static builds move client assets out of `dist/.prerender/` before
                // rendering pages, so build-time code would load WASM from a URL
                // that's gone.
                //
                // Inlining it avoids the lookup. The client still loads WASM as
                // its own file.
                //
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

      /**
       * Rebuild on Rust edits during `astro dev`
       */
      'astro:server:setup': ({ server, logger }) => {
        let queued = false
        let latest = Promise.resolve()

        // Saving several files at once fires several events.
        //
        // Overlapping builds would race to write the same output.
        //
        function rebuild() {
          if (queued) return

          queued = true
          latest = latest.then(async () => {
            queued = false
            const files = await buildInDev(logger)
            for (const file of files) server.watcher.emit('change', file)
          })
        }

        server.watcher.on('all', (_event, file) => {
          if (/\.rs$|\/Cargo\.(toml|lock)$/.test(file)) rebuild()
        })
      },
    },
  }
}

/**
 * Builds for `astro dev`, logging failures instead of throwing
 *
 * Broken Rust never stops the dev server, only builds and checks. The last good
 * bindings stay in place, so pages keep working until the next good build.
 *
 */
async function buildInDev(logger: AstroIntegrationLogger): Promise<string[]> {
  try {
    return await build('dev', logger)
  } catch (error) {
    logger.error(String(error))
    return []
  }
}

/**
 * Builds every WASM crate and its bindings
 *
 * One Cargo run covers all crates, so Cargo can share work between them.
 *
 * Returns the files written.
 *
 */
async function build(profile: Profile, logger: AstroIntegrationLogger): Promise<string[]> {
  const start = performance.now()
  const crates = await findWasmCrates()

  // Without any `--package`, Cargo would build the whole workspace
  if (crates.length === 0) return []

  const packageArgs = crates.map(({ name }) => `--package=${name}`)
  await exec('cargo', [
    'build',
    '--quiet',
    '--target=wasm32-unknown-unknown',
    `--profile=${profile}`,
    ...packageArgs,
  ])
  const bindings = await Promise.all(crates.map((crate) => generateBindings(crate, profile)))

  const names = crates.map(({ name }) => name).join(', ')
  logger.info(`Built ${names} (${profile}) in ${Math.round(performance.now() - start)} ms`)

  return bindings.flat()
}

/**
 * Finds the workspace crates to build for WASM
 */
async function findWasmCrates(): Promise<Crate[]> {
  const { stdout } = await exec('cargo', ['metadata', '--no-deps', '--format-version=1'])
  const { packages } = JSON.parse(stdout) as { packages: Package[] }

  return packages.flatMap(({ name, dependencies, targets }) => {
    const usesWasmBindgen = dependencies.some((d) => d.name === 'wasm-bindgen')
    const lib = targets.find(({ kind }) => kind.includes('cdylib'))

    return usesWasmBindgen && lib ? [{ name, libName: lib.name }] : []
  })
}

/**
 * Generates a compiled crate's JS bindings and types
 *
 * They go in `target/wasm-bindgen/<crate>/`, where `#rust/<crate>` resolves.
 *
 * Returns the files written, so dev rebuilds can announce them to Vite.
 *
 */
async function generateBindings({ name, libName }: Crate, profile: Profile): Promise<string[]> {
  const wasm = `${targetDir}wasm32-unknown-unknown/${profile === 'dev' ? 'debug' : profile}/${libName}.wasm`
  const outDir = `${targetDir}wasm-bindgen/${name}`
  const wasmFile = `${outDir}/index_bg.wasm`

  await exec('wasm-bindgen', ['--target=bundler', '--out-name=index', `--out-dir=${outDir}`, wasm])

  if (profile === 'wasm-release') {
    await exec('wasm-opt', ['-Oz', ...wasmOptFeatures, wasmFile, '-o', wasmFile])
  }

  const files = await readdir(outDir)
  return files.map((file) => `${outDir}/${file}`)
}

/**
 * Cargo profile for WASM builds
 *
 * - `dev` keeps rebuilds fast in `astro dev`
 * - `wasm-release` optimizes for size everywhere else
 *
 */
type Profile = 'dev' | 'wasm-release'

/**
 * A crate to build for WASM
 */
interface Crate {
  name: string

  /**
   * Names the compiled file
   *
   * Uses underscores where the package name may use hyphens.
   *
   */
  libName: string
}

/**
 * The parts of a `cargo metadata` package used here
 */
interface Package {
  name: string
  dependencies: { name: string }[]
  targets: { name: string; kind: string[] }[]
}

/**
 * Cargo's target directory, which `#rust/*` in `package.json` also points into
 */
const targetDir = fileURLToPath(new URL('../../target/', import.meta.url))

/**
 * wasm-opt flags for Rust's default WASM features
 *
 * wasm-opt can't detect which features Rust's output uses, and rejects any it
 * hasn't been told about.
 *
 * Rust lists its defaults with:
 *
 *     rustc --print cfg --target wasm32-unknown-unknown
 *
 */
const wasmOptFeatures = [
  'bulk-memory',
  'multivalue',
  'mutable-globals',
  'nontrapping-float-to-int',
  'reference-types',
  'sign-ext',
].map((feature) => `--enable-${feature}`)

/**
 * Runs a command without a shell
 *
 * A failed command rejects with its stderr in the error message, which is how
 * Cargo's errors reach the terminal.
 *
 */
const exec = promisify(execFile)
