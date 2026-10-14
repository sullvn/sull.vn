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
      /**
       * Build before anything imports the bindings
       *
       * This hook runs before Vite starts, for `dev`, `build`, and `sync`. As
       * `astro check` runs `sync` first, the bindings' types exist before it
       * type-checks. `preview` only serves `dist/`, so it skips the build.
       *
       */
      'astro:config:setup': async ({ command, updateConfig, logger }) => {
        if (command === 'preview') return

        updateConfig({
          vite: {
            plugins: [
              {
                // Static builds move client assets out of `dist/.prerender/` before
                // rendering pages, so build-time code would load wasm from a URL
                // that's gone. Inlining it avoids the lookup, and the client still
                // loads wasm as its own file.
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
       *
       * This hook is the only one with the dev server's file watcher. It already
       * watches the whole project, so new crates and files are covered without a
       * second watcher.
       *
       */
      'astro:server:setup': ({ server, logger }) => {
        let queued = false
        let latest = Promise.resolve()

        // Saving several files at once fires several events, and overlapping
        // builds would race to write the same output. Chaining runs one build at
        // a time, and the flag folds every change during a build into one more.
        function rebuild() {
          if (queued) return
          queued = true
          latest = latest.then(async () => {
            queued = false
            const files = await buildInDev(logger)
            // Vite ignores `target/`, which is gitignored, so it wouldn't notice
            // the new bindings and reload the pages using them by itself
            for (const file of files) server.watcher.emit('change', file)
          })
        }

        // Only Rust sources and manifests, so the announcements above can't
        // start another build
        server.watcher.on('all', (_event, file) => {
          if (/\.rs$|\/Cargo\.(toml|lock)$/.test(file)) rebuild()
        })
      },
    },
  }
}

/**
 * Cargo profile for wasm builds
 *
 * `dev` keeps rebuilds fast in `astro dev`. `wasm-release`, set up in
 * `Cargo.toml`, optimizes for size everywhere else.
 *
 */
type Profile = 'dev' | 'wasm-release'

/**
 * A crate to build for wasm
 *
 * `libName` names its compiled file, which uses underscores where the package
 * name may use hyphens.
 *
 */
interface Crate {
  name: string
  libName: string
}

/** The parts of a `cargo metadata` package used here */
interface Package {
  name: string
  dependencies: { name: string }[]
  targets: { name: string; kind: string[] }[]
}

/** Cargo's target directory, which `#rust/*` in `package.json` also points into */
const targetDir = fileURLToPath(new URL('../../target/', import.meta.url))

/**
 * wasm-opt flags for Rust's default wasm32 target features
 *
 * wasm-opt can't detect which features Rust's output uses, and rejects any it
 * hasn't been told about. Rust lists its defaults with
 * `rustc --print cfg --target wasm32-unknown-unknown`.
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

/**
 * Finds the workspace crates to build for wasm
 *
 * Those are crates using wasm-bindgen with a `cdylib`, the crate type that
 * produces a `.wasm` file. Other crates may be native-only and might not even
 * compile for wasm.
 *
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
 * Returns the files written, so dev rebuilds can announce them to Vite.
 *
 */
async function generateBindings({ name, libName }: Crate, profile: Profile): Promise<string[]> {
  // Cargo writes dev builds to debug/
  const wasm = `${targetDir}wasm32-unknown-unknown/${profile === 'dev' ? 'debug' : profile}/${libName}.wasm`
  const outDir = `${targetDir}wasm-bindgen/${name}`
  const wasmFile = `${outDir}/index_bg.wasm`

  await exec('wasm-bindgen', ['--target=bundler', '--out-name=index', `--out-dir=${outDir}`, wasm])

  // Optimize wasm-bindgen's output, not Cargo's, as wasm-bindgen rewrites the
  // module. Dev output is never shipped, so it isn't worth the extra step.
  if (profile === 'wasm-release') {
    await exec('wasm-opt', ['-Oz', ...wasmOptFeatures, wasmFile, '-o', wasmFile])
  }

  const files = await readdir(outDir)
  return files.map((file) => `${outDir}/${file}`)
}

/**
 * Builds every wasm crate and its bindings
 *
 * One Cargo run covers all crates, so Cargo can share work between them.
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
