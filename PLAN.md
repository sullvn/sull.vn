# Rust → wasm for the Astro site

## Goal

The `rust-integration` branch prepares the site for Rust → wasm code. It should end with:

1. A Rust workspace with config and lints set up to be as modern and strict as possible.
2. A working Rust → wasm integration for Astro.
3. A test crate and a site page that exercise the full pipeline.

Everything should be as simple, ergonomic, understandable, modern, and strict as possible.

Decisions so far:

- Test page lives at `/rust-wasm-test`.
- The workspace will also hold native crates, so nothing assumes every crate targets wasm: the integration builds only crates that depend on wasm-bindgen, lints suit native code too, and size settings live in a wasm-only profile.
- Use the latest stable Rust, pinned by `flake.lock`.
- Update the project flake as needed.
- No supply-chain checks (`cargo-deny`, `cargo-shear`) yet.
- Keep CI and deploys fast. Cloudflare stops building the site; GitHub Actions builds, tests, and deploys `dist/` with wrangler.
- No two tools doing the same job (e.g. no second Nix cache unless measurements justify it).

## Implementation steps (short)

1. **Update dependencies:** `nix flake update`, bump astro to 7.3.8 (brings Vite's security fix).
2. **CI and deploys:** run CI on pushes to `main` too (so caches are shared), deploy previews and production from GitHub Actions with `cloudflare/wrangler-action`, turn off Cloudflare's own builds.
3. **Rust workspace and test crate:** rust-overlay toolchain from `rust-toolchain.toml` (`channel = "stable"`), `wasm-bindgen-cli` matched to `Cargo.lock`, workspace `Cargo.toml` with strict lints and a `wasm` profile, `clippy.toml`, `crates/smoke` with native tests, `test:cargo` script, `Swatinem/rust-cache` in CI.
4. **Astro integration:** `src/integrations/rust.ts` builds the wasm-bindgen crates with `cargo` + `wasm-bindgen`, fixes Astro's prerender wasm bug, rebuilds on Rust edits in dev; `#rust/*` import alias.
5. **Test page:** `/rust-wasm-test` uses `crates/smoke` at build time and in a client `<script>`; Playwright test in Chromium, Firefox, and WebKit.

Then, only after measuring: [optional extras](#optional-explore-after-measuring).

## Implementation steps (detailed)

### 1. Update dependencies

- `nix flake update`: nixpkgs moves from 2026-10-05 to its current head, which has `wasm-bindgen-cli` 0.2.129.
- Bump astro to 7.3.8, which requires Vite 8.3.x and includes the fix for GHSA-vfpm-58rq-9qcg.
- Run the existing tests to confirm the visual snapshots don't change.

### 2. CI and deploys

Useful on its own, before any Rust: the existing Astro image cache is never shared between PRs today, and moving the build out of Cloudflare is what lets Rust deploy at all.

#### `.github/workflows/checks.yaml`

- **Triggers:** add `push: branches: [main]` alongside `pull_request`. A PR can only restore caches saved by its own runs or by `main`, so the `main` run is what fills shared caches.
- **Concurrency:** keep the group, but only cancel in-progress runs on PRs, so a production deploy is never cut off: `cancel-in-progress: ${{ github.event_name == 'pull_request' }}`.
- **Permissions:** add `deployments: write` (for the GitHub Deployment wrangler-action creates).
- **Deploy step**, after `Tests` (Playwright's `webServer` already runs `astro build`, so `dist/` exists and is exactly what was tested):

  ```yaml
  - name: Deploy
    if: github.event_name == 'push' || !github.event.pull_request.head.repo.fork
    uses: cloudflare/wrangler-action@<sha> # v4.1.3
    with:
      apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
      accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
      gitHubToken: ${{ secrets.GITHUB_TOKEN }}
      command: pages deploy dist --project-name=sullvn --branch=${{ github.head_ref || github.ref_name }}
  ```

  - `main` deploys production; any other branch gets a `<branch>.sullvn.pages.dev` preview.
  - The `if` skips fork PRs, which can't read secrets. The repo has no Dependabot config; if one is added, its PRs also can't read secrets and need `github.actor != 'dependabot[bot]'`.
  - Deploys only happen when tests pass.
  - Deploying from the test job, rather than a separate one, avoids passing `dist/` between jobs as an artifact.

#### Manual steps (owner)

1. Create a Cloudflare API token with **Account > Cloudflare Pages > Edit**.
2. Add `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID` as repo secrets.
3. After the new workflow has deployed successfully once, turn off Cloudflare's builds in Pages > Settings > Build > Branch control: untick automatic production deployments, set preview branches to None. The `sullvn` project, its domain, and URLs stay the same.

Trade-off: the Cloudflare bot's PR comment and check run become a GitHub Deployment link.

### 3. Rust workspace and test crate

#### `flake.nix`

Add the rust-overlay input, with its nixpkgs following ours. No overlay: `rust-overlay.lib.mkRustBin { } pkgs` gives the same `rust-bin` on the existing `nixpkgs.legacyPackages`, so `pkgs` doesn't need re-importing with `overlays` (tested: evaluates to Rust 1.99.0 on aarch64-linux). Add to the devShell:

```nix
let
  inherit (pkgs) lib;
  rust-bin = rust-overlay.lib.mkRustBin { } pkgs;
  # wasm-bindgen-cli must match the wasm-bindgen crate in Cargo.lock exactly.
  # Evaluation fails if nixpkgs has no package for that version.
  wasmBindgen = lib.findFirst (p: p.name == "wasm-bindgen") (throw "wasm-bindgen is not in Cargo.lock")
    (lib.importTOML ./Cargo.lock).package;
  wasm-bindgen-cli = pkgs."wasm-bindgen-cli_${lib.replaceStrings [ "." ] [ "_" ] wasmBindgen.version}";
in
# ...
packages = [
  # ...existing packages
  (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
  wasm-bindgen-cli
];
```

`Cargo.lock` must be committed (and tracked by git) for the flake to read it. Reading the version from `Cargo.lock` means it lives in one place; the cost is that the lockfile must be tracked, `.envrc` must watch it, and a crate depending on wasm-bindgen must exist from this step on.

**Update flow:** `nix flake update` brings the latest stable Rust (rust-overlay) and the latest `wasm-bindgen-cli` (nixpkgs). Then `cargo update -p wasm-bindgen --precise <nixpkgs version>`. If `Cargo.lock` names a version nixpkgs lacks (say, after a plain `cargo update`), `nix develop` fails with an error listing the versions nixpkgs has.

#### `rust-toolchain.toml`

```toml
[toolchain]
channel = "stable"
components = ["clippy", "rustfmt", "rust-analyzer", "rust-src"]
targets = ["wasm32-unknown-unknown"]
profile = "minimal"
```

`targets` adds wasm32 on top of the host target, which is always included, so native `cargo build` and `cargo test` work on any machine.

#### `Cargo.toml` (repo root, next to `package.json`)

- `[workspace]`: `members = ["crates/*"]`, `resolver = "3"`.
- `[workspace.package]`: `edition = "2024"`. No `rust-version`. No `version`, which also makes every crate unpublishable (`"publish": []`; `cargo publish` refuses, tested), so no `publish = false`.
- `[workspace.dependencies]`: `wasm-bindgen = "0.2"`. `Cargo.lock` is the only place the exact version lives, and the flake reads it from there. A minimum like `"0.2.129"` would repeat it, and would block `cargo update --precise` to an older version when nixpkgs lags.
- `[profile.wasm]`: `inherits = "release"`, `opt-level = "s"`, `lto = true`, `codegen-units = 1`, `strip = true`. A custom profile, so native release builds keep Cargo's defaults (`opt-level = 3`); the integration builds with `--profile wasm` (tested; with `--target`, output lands in `target/wasm32-unknown-unknown/wasm/`). `strip` matters: without it the final wasm was 46% larger (see research).
- Lints: groups wherever one exists, single lints only where no group covers them.

  ```toml
  [workspace.lints.rust]
  let_underscore = { level = "warn", priority = -1 }
  rust_2018_idioms = { level = "warn", priority = -1 }
  unused = { level = "warn", priority = -1 } # also enables the off-by-default unused_* lints
  missing_debug_implementations = "warn"
  missing_docs = "warn"
  unreachable_pub = "warn"
  unsafe_code = "forbid"
  unused_qualifications = "warn"

  [workspace.lints.clippy]
  pedantic = { level = "warn", priority = -1 }
  nursery = { level = "warn", priority = -1 }
  missing_const_for_fn = "allow" # wasm-bindgen rejects `const fn`, and workspace lints apply to every crate
  redundant_pub_crate = "allow"  # contradicts `unreachable_pub`
  # Restriction lints, cherry-picked (the group as a whole contradicts itself).
  # Hidden panics, exits, and silent truncation (on wasm, a panic traps with no message):
  as_conversions = "warn"
  exit = "warn"
  expect_used = "warn"
  indexing_slicing = "warn"
  missing_assert_message = "warn"
  panic = "warn"
  panic_in_result_fn = "warn"
  string_slice = "warn"
  unreachable = "warn"
  unwrap_used = "warn"
  # `#[expect]` with a reason instead of `#[allow]`:
  allow_attributes = "warn"
  allow_attributes_without_reason = "warn"
  # Debug leftovers:
  dbg_macro = "warn"
  print_stderr = "warn"
  print_stdout = "warn"
  todo = "warn"
  unimplemented = "warn"
  # Style:
  clone_on_ref_ptr = "warn"
  empty_structs_with_brackets = "warn"
  if_then_some_else_none = "warn"
  mod_module_files = "warn"
  redundant_type_annotations = "warn"
  str_to_string = "warn"
  try_err = "warn"
  unused_result_ok = "warn"
  ```

  - **Groups:** rustc has no pedantic or "all" group, only topic groups, so the useful lints that belong to none are listed by hand. `rust_2018_idioms` is still the latest idioms group; 2021 and 2024 added none, and `rust_2024_compatibility` only helps migrate older code to 2024. Not clippy's `cargo` group: for unpublished crates its only lint that fires is `multiple_crate_versions`, which wasm-bindgen's own dependencies trigger.
  - **`priority = -1` on every group, none on single lints.** Cargo orders the flags by priority and later flags win, so single lints override groups. Clippy's `lint_groups_priority` (an error by default) rejects any group at the same priority as any single lint in the table, even one outside the group.
  - **Levels:** `warn`, so a half-finished edit still builds and reloads, and false positives can take a local `#[expect(clippy::x, reason = "...")]`. `CARGO_BUILD_WARNINGS=deny` turns every warning into an error in `pnpm test` and CI. `unsafe_code` is `forbid`, which no `#[allow]` can override; if a crate ever needs `unsafe`, change it to `deny` and add `#[expect(unsafe_code, reason = "...")]` where needed.
  - **Restriction lints:** 25, grouped by purpose with a comment per group, chosen to suit native crates as well as wasm ones. `clippy.toml` relaxes `unwrap`, `expect`, `panic`, and indexing in tests; `unreachable` and `exit` have no such setting, but rarely appear in tests. `exit` doesn't fire in `main`.
  - **Tests under these lints:** `missing_assert_message` and `panic_in_result_fn` also apply inside tests, and `clippy.toml` has no setting to relax either (tested for `panic_in_result_fn`). So every assertion carries a message, and a test that asserts doesn't return `Result`: it uses `.unwrap()` or `.expect()`, which tests allow, instead of `?`. If this proves noisy, relax them later.
  - **Binaries:** `print_stdout`/`print_stderr` steer CLI output to `writeln!` on a locked `io::stdout()`, which returns an error on a closed pipe instead of panicking. A binary that prints a lot can take a crate-level `#![expect(clippy::print_stdout, reason = "...")]`.
  - **Left out:** `tests_outside_test_module` fires on every test in a crate's `tests/` directory (tested), which native crates use for integration tests.
  - **Overlaps aren't tracked.** rustc and clippy sometimes flag the same code twice; that's harmless and changes between releases. When two lints contradict each other, allow one with a comment saying why, like the two `allow`s above.

#### `crates/smoke`

```toml
[package]
name = "smoke"
edition.workspace = true

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen.workspace = true

[lints]
workspace = true
```

One export per kind of value that crosses the boundary:

- `greet`: strings
- `sum(&[f64])`: typed arrays in, numbers out
- `parse_byte`: `Result<u8, JsError>`, thrown as an exception in JS
- `Counter`: a class with methods, for state

**Thin wrappers.** Logic lives in plain Rust functions; `#[wasm_bindgen]` exports only convert at the boundary. Native tests can't call anything that creates a `JsValue` or `JsError`, so the logic must be reachable without them. `JsError` implements `From<E: std::error::Error>`, so a fallible export is one line:

```rust
/// Parses a decimal byte.
///
/// # Errors
///
/// If `s` is not a number from 0 to 255.
#[wasm_bindgen]
pub fn parse_byte(s: &str) -> Result<u8, JsError> {
    Ok(parse(s)?)
}

fn parse(s: &str) -> Result<u8, ParseIntError> {
    s.parse()
}
```

Unit tests live in a `#[cfg(test)] mod tests` at the bottom of each file, plus doctests. They run natively via `cargo test`.

**Who tests what:**

- `cargo test` (native): the logic. Fast, no browser or Node.
- Playwright (step 5): the boundary, meaning the real build crossing into JS in three engines, including errors arriving as JS exceptions and class state.
- No `wasm-bindgen-test` for now; add it if crates later get heavy `web-sys` / `js-sys` code that needs fast Rust-level boundary tests.

#### Other files

- No `rustfmt.toml`: every entry point (`cargo fmt`, rust-analyzer) passes the crate's edition, and every useful option is nightly-only. The `rustfmt` toolchain component stays, since `cargo fmt` runs it.
- `clippy.toml`: `allow-unwrap-in-tests`, `allow-expect-in-tests`, `allow-panic-in-tests`, `allow-indexing-slicing-in-tests`, `avoid-breaking-exported-api = false`.
- `.gitignore`: `target/`, `crates/*/pkg/`. Biome already honours `.gitignore` (`vcs.useIgnoreFile`), so it skips both.
- `tsconfig.json`: add `target` to `exclude`, next to `dist`. Its `include` is `**/*`, so TypeScript would otherwise walk the whole of `target/`, which has no TypeScript in it. `pkg/` stays included, since its `.d.ts` files are the types.
- `.envrc`: `watch_file Cargo.lock`, since the devShell depends on it and nix-direnv only watches `flake.nix`, `flake.lock`, and `devshell.toml`.

#### `package.json` scripts

- `test:cargo`, added to `test` before `test:astro`, so lint and native test failures show before `astro check` compiles the crates. pnpm runs scripts with `sh`, so one `export` covers every command:

  ```json
  "test:cargo": "export CARGO_BUILD_WARNINGS=deny && cargo fmt --check && cargo clippy --all-targets && cargo test"
  ```

  - One native clippy run covers every crate, native and wasm, including test code (`--all-targets`). A wasm32 run would fail on native-only crates, and on our wasm crates it reported the same as the native run (tested); wasm-bindgen's generated code isn't linted on either target. Add a wasm32 run for wasm crates only if they gain `cfg(target_arch = "wasm32")` code.
  - No wasm build here: `astro check` and Playwright's `astro build` build the crates through the integration.
  - No `cargo doc`: nobody reads the rustdoc HTML. Doc comments reach readers through wasm-bindgen's `.d.ts` and editor hovers, and `missing_docs`, clippy's doc lints, and doctests already check them. `cargo doc` would only add broken intra-doc link checks, at the cost of another compile.
- `format`: also runs `cargo fmt`.

#### CI caching

Add `Swatinem/rust-cache` (v2.9.2, pinned by SHA) to `checks.yaml`, after `Install Nix`:

```yaml
- name: Cache Rust
  uses: Swatinem/rust-cache@<sha> # v2.9.2
  with:
    cmd-format: nix develop --no-update-lock-file -c {0}
    save-if: ${{ github.ref == 'refs/heads/main' }}
```

- Caches `~/.cargo` and compiled dependencies in `target/`, keyed on the Rust version, `Cargo.lock`, `Cargo.toml`, `rust-toolchain.toml`, and `CARGO*`/`RUST*` env vars. Sets `CARGO_INCREMENTAL=0` itself.
- `save-if` means only `main` saves; PRs restore `main`'s cache and don't fill the 10 GB limit.

Expected CI time: about 75–90 s (vs. about 67 s today, and about 105–110 s with Rust and no caching). See research for the breakdown.

### 4. Astro integration

#### `package.json`

```jsonc
"imports": { "#rust/*": "./crates/*/pkg/index.js" }   // import { greet } from '#rust/smoke'
```

#### `src/integrations/rust.ts`

About 50 lines, registered in `astro.config.ts`. `src/integrations/` is where the site's last integration (`exportRoutes.ts`) lived.

- **Build on `astro:config:setup`.** Runs for `dev`, `build`, and `sync` (which `astro check` runs first), so the generated package and its types exist before anything reads them. Skipped for `preview`.
  1. `cargo metadata --no-deps --format-version=1` lists the workspace's packages; the wasm crates are the ones that depend on `wasm-bindgen` (tested). If there are none, stop: a bare `cargo build` would build the whole workspace.
  2. `cargo build --target wasm32-unknown-unknown --message-format=json-render-diagnostics -p <crate>…` (`--profile wasm` unless `dev`, which stays on debug so rebuilds stay fast as crates grow; `sync` builds the wasm profile too, so in `pnpm test` and CI the later `astro build` reuses `astro check`'s compile). Only wasm crates, because a native crate may not compile for wasm32. Cargo prints errors to stderr in its usual human format and only JSON messages on stdout (tested). Each `compiler-artifact` line with a `.wasm` filename is a cdylib crate, so a wasm-bindgen helper library without one is built but skipped; no crate-type filtering, no `-` → `_` name mapping.
  3. `wasm-bindgen --target bundler --out-name index --out-dir <dirname(manifest_path)>/pkg` for each, on every build. It takes under 0.1 s per crate. The cost is an unneeded page reload in dev when an edit leaves a wasm crate unchanged (e.g. an edit to a native crate). If that gets annoying, skip artifacts cargo reports as `"fresh": true`, but only on dev rebuilds: after `astro dev`, `pkg/` holds debug output, and a later `astro build` whose wasm-profile artifact is still fresh would otherwise ship it.
- **Prerender fix.** A `configEnvironment` plugin inlines `.wasm` for every environment except `client`. Written as one expression, which also avoids TS 7030 without an explicit `return undefined`:

  ```ts
  configEnvironment: (name) =>
    name === 'client' ? undefined : { build: { assetsInlineLimit: (file) => file.endsWith('.wasm') || undefined } },
  ```

  File the Astro bug upstream and link it from a comment.
- **Dev rebuilds on `astro:server:setup`.** On any `*.rs`, `Cargo.toml`, or `Cargo.lock` change from Vite's watcher (which already watches the project, so `build.rs` and new crates are covered too), rebuild without blocking the dev server. A rebuild runs the same three steps, so new crates are picked up. Two flags coalesce saves: `building` is set during a build, and a change while it's set only sets `dirty`; when the build finishes, it runs once more if `dirty` is set. No queue, no debounce timer. Cargo's errors reach the terminal through stderr. In `dev`, a failed build is only logged and the page keeps the last good build, so broken Rust never stops the dev server, even at startup. Elsewhere a failed build throws, so `astro build` and `astro check` fail. Vite picks up the regenerated `pkg/` and reloads.
- **Config:** add `${root}target/**` to `vite.server.watch.ignored`, matching the existing `${root}**/dist/**` entry. The workspace has a single `target/`, at the root.

Types come from wasm-bindgen's generated `index.d.ts`: no `.rs` imports, no hand-written type files, no wasm-pack, no wasm plugins.

**Usage rules** (from the prior design's testing), kept as a comment at the top of `src/integrations/rust.ts` so they outlive this plan:

- No browser APIs in code that runs at build time (frontmatter, server render of `client:load` islands). Shared functions should take data and return plain values.
- Always call imported functions directly; hot updates replace the module, so cached references go stale.
- Frontmatter and client imports are separate instances that share no memory; pass state through the HTML.

### 5. Test page and end-to-end test

- **`src/pages/rust-wasm-test.astro`** uses `crates/smoke` two ways, which between them cover build time and the browser:
  - in frontmatter, baked into the HTML at build time, including the thrown error's message
  - in a plain `<script>`, which wires a `Counter` to a button: the same Vite wasm import, loaded in the browser
  - No Svelte island: the site has no `.svelte` files yet, and an island's server render runs in the same environment as the frontmatter. Add one when real wasm code lives in islands.
- **`tests/rust-wasm.spec.ts`** checks the build-time HTML, clicks the counter, and checks the error message. Runs as three new Playwright projects, one expression next to `visualCases.map`:

  ```ts
  ...(['chromium', 'firefox', 'webkit'] as const).map((browserName) => ({
    name: `rust-wasm-${browserName}`,
    testMatch: 'rust-wasm.spec.ts',
    use: { browserName },
  })),
  ```

  The existing visual projects get `testMatch: 'visual.spec.ts'`.

### Optional: explore after measuring

Only once real CI timings or real crates exist, and only if they justify the extra moving part:

- **`wasm-opt`** (binaryen, 90 MB in the devShell). Saved about 11% (1.7 KB) on the toy crate. If added: `wasm`-profile builds only, `-Os` with the six default Rust wasm target features enabled explicitly (see research).
- **`.cargo/config.toml` with `build.target = "host-tuple"`.** Lets proc-macros and build scripts compile once and be shared between native and wasm32 commands; `target/` shrank from 351 MB to 223 MB. With rust-cache, cold builds only happen when dependencies change, and with clippy and tests both native the sharing gain is smaller. Costs a file and moves native output to `target/<host-tuple>/`. Not `target = "wasm32-unknown-unknown"`: it would make plain `cargo test` build for wasm.
- **`DeterminateSystems/magic-nix-cache-action`** (v15). Caches Nix store paths that aren't on cache.nixos.org, which here is just the rust-overlay toolchain. It doesn't overlap with anything existing: `cachix/install-nix-action` only installs Nix, and cache.nixos.org keeps serving everything nixpkgs builds. Estimated saving 10–15 s, unverified. Measure the toolchain setup time on the first runs; add it only if that's a meaningful share.
- **`CARGO_PROFILE_DEV_DEBUG=0`** as a CI env var: smaller `target/`, faster linking. rust-cache folds it into its key automatically.
- **Workers static assets** instead of Pages (`wrangler deploy` with an `assets` directory, `wrangler preview` for PRs). Cloudflare now steers new projects to Workers, but it's a separate migration (URLs move to workers.dev) and not needed for Rust.

## Research

Researched 2026-10-12, by four parallel research passes (Rust workspace, wasm toolchain, Vite/Astro integration, Nix) plus follow-up prototypes. Versions and behaviour were checked against primary sources and, where noted, by running real builds in scratch projects on this machine (aarch64-linux, same arch as CI). Facts that don't bear on a decision are left out.

### Ecosystem status

- **rustwasm org sunset:** announced 2025-07-21, archived September 2025. https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org/
- **wasm-bindgen** moved to https://github.com/wasm-bindgen/wasm-bindgen. Latest **0.2.129** (2026-09-25), released every 2–5 weeks; the 0.2.x series still has small breaking changes (0.2.122, 0.2.123). Library MSRV 1.81. Companion crates (js-sys, web-sys, wasm-bindgen-futures, wasm-bindgen-test) release in lockstep. 0.2.129 pulls in `syn v3`, so duplicate syn 2/3 versions are likely as crates are added.
- **wasm-pack** (v0.15.0, 2026-05-15) is maintained. What it adds, from its source: reads the wasm-bindgen version from `Cargo.lock`, uses a matching `wasm-bindgen` on `PATH` or downloads one, downloads binaryen and runs `wasm-opt` on release builds, and writes npm `package.json`/README. In a Nix setup the downloads are a downside and the rest is a few lines in the integration.
- **console_error_panic_hook** is archived (last release 2021); see Panics.
- **Component model / jco:** active, but for browser + Node it means WIT files, WASI shims, and a transpile step; DOM access is awkward. Not the simpler option today.

### Rust toolchain and language

- Current stable: **1.99.0** (2026-10-01). Edition **2024** is the latest; no 2027 edition is announced.
- 1.97 stabilized Cargo `build.warnings` (`CARGO_BUILD_WARNINGS=deny`). 1.98 made `unsafe_code` fire consistently for unsafe attributes (re-tested against wasm-bindgen; still fine). 1.99 turns off incremental builds when `CI` is set.
- **Resolver 3** is the edition-2024 default but a virtual workspace has no package edition, so set it explicitly. Without `rust-version`, resolution and clippy's MSRV use the toolchain in use. https://doc.rust-lang.org/cargo/reference/resolver.html
- **`CARGO_BUILD_WARNINGS=deny`** beats `RUSTFLAGS=-Dwarnings`: RUSTFLAGS invalidates the build cache, applies to dependencies' build scripts, and overrides config rustflags. Tested: fails `cargo clippy` (exit 101) and fails `cargo doc` on a broken intra-doc link, so `RUSTDOCFLAGS` isn't needed. Kept out of config so local work isn't blocked.
- Cargo's `[lints.cargo]` is still nightly-only. Lints aren't inherited implicitly; each crate needs `[lints] workspace = true`.
- `#[expect]` and `reason = "..."` stable since 1.81. `clippy::allow_attributes` with `allow_attributes_without_reason` was tested: both fire on a bare `#[allow]`.
- **rustfmt:** `imports_granularity`, `group_imports`, `reorder_impl_items`, `wrap_comments`, `format_code_in_doc_comments` are all nightly-only. `style_edition` defaults to 2015 for bare `rustfmt`, but `cargo fmt` and rust-analyzer pass the crate's edition, so no `rustfmt.toml` is needed.
- **clippy.toml:** unknown keys error, so all chosen keys are verified accepted by clippy 1.99. `check-private-items = true` made `missing_panics_doc` fire on `assert_eq!` inside tests; left off.
- **rust-toolchain.toml:** plain `"stable"` with rustup silently changes lints every 6 weeks; with rust-overlay, `flake.lock` pins what "stable" means.
- wasm crates need `crate-type = ["cdylib", "rlib"]`; `rlib` lets native tests and doctests work.

### Strict lints vs. wasm-bindgen

Tested on Rust 1.95 and 1.99, native and `--target wasm32-unknown-unknown`, exercising free functions, a struct with an impl block, getters/setters, `pub` fields, a C-style enum, an `extern "C"` import, `Result<_, JsError>`, `Option<String>`, `&JsValue`, `Vec<f64>` and `&[f64]`, `typescript_custom_section`, and `#[wasm_bindgen(start)]`.

- **`unsafe_code = "forbid"` works.** The expansion contains 22 `unsafe` tokens plus `#[no_mangle]`, `#[export_name]`, and `#[link_section]`, yet nothing fires. A hand-written `unsafe {}` does error.
- **No noise from generated code** with `missing_docs`, `missing_debug_implementations`, `unreachable_pub`, `unused_qualifications`, pedantic, nursery, or the chosen restriction lints. The full set passed with zero warnings on native and wasm32.
- **`clippy::missing_const_for_fn`** suggests `const fn`, but then the macro fails: `error: can only #[wasm_bindgen] non-const functions`.
- **`Ok(parse(s)?)`** into `Result<u8, JsError>` passes the full lint set (no `needless_question_mark`, since the error type changes). Rust 1.99, wasm-bindgen 0.2.129.
- Left out as noisy: `missing_inline_in_public_items` (fires on every export), `unused_crate_dependencies` (fires for dev-dependencies), `exhaustive_structs`, `std_instead_of_core`/`alloc` (only for `no_std`), `missing_docs_in_private_items`, `arithmetic_side_effects`, `min_ident_chars`, `pub_use`, `shadow_unrelated`, `single_char_lifetime_names`, `wildcard_enum_match_arm`. Enabling all of `clippy::restriction` is discouraged by clippy itself.
- **Restriction lints from r/rust** (https://www.reddit.com/r/rust/comments/1m0r3s7/do_you_use_clippyrestriction_lints/, read via the PullPush archive 2026-10-13): most of what commenters use is already here (`clone_on_ref_ptr`, `if_then_some_else_none`, `str_to_string`, `try_err`, `as_conversions`, `dbg_macro`, `print_*`, `unwrap_used`/`expect_used`, `indexing_slicing`, `allow_attributes_without_reason`, `mod_module_files`). The rest, tested with clippy 1.95 on a crate with wasm-bindgen exports, fired on none of wasm-bindgen's generated code. Left out: `absolute_paths` (fires on idiomatic `std::mem::forget`-style calls), `pattern_type_mismatch` (fights match ergonomics), `shadow_reuse` (fires on `let s = s.trim()`), `impl_trait_in_params` (taste; wasm-bindgen exports can't take `impl Trait` anyway), `mem_forget` (wasm-bindgen leaks through `Closure::forget`, which it doesn't flag), the `unsafe`/safety-comment lints (`unsafe_code` is forbidden), `exhaustive_enums` (nothing is published), `std_instead_of_*` and `missing_docs_in_private_items` (above). Adopted: `unreachable` (the `panic` lint doesn't cover `unreachable!`), `exit` (flags `process::exit` outside `main`), and `panic_in_result_fn` (flags `assert!` and `panic!` in functions that return `Result`; it also fires in tests, with no `clippy.toml` exemption, so tests that assert don't return `Result`).
- `needless_pass_by_value` (pedantic) correctly flags `Vec<f64>` params; `&[f64]` works with wasm-bindgen.
- `clippy::string_to_string` has been removed (merged into `implicit_clone`).
- **The groups-first config** (step 3) passed on a crate with `greet`, `parse_byte`, a `Counter` class, a private module with a `pub(crate)` fn, and a test module, native and wasm32 (Rust 1.99, wasm-bindgen 0.2.129). The `unused` group added no noise from generated code; its only warning was a deliberately unused macro rule (`unused_macro_rules`, off by default but in the group).

### Lint configuration

- **rustc lint groups** (`rustc -W help`, 1.99): `warnings` (only lints already at warn), `deprecated_safe`, `future_incompatible`, `keyword_idents`, `let_underscore` (`let_underscore_drop`, `let_underscore_lock`), `nonstandard_style`, `refining_impl_trait`, `rust_2018_compatibility`, `rust_2018_idioms`, `rust_2021_compatibility`, `rust_2024_compatibility`, `unknown_or_malformed_diagnostic_attributes`, `unused`. No pedantic or "everything" group, and no idioms group after 2018. Setting a group's level sets it for every member, including off-by-default ones like `unused_macro_rules`.
- **`rust_2018_idioms` on edition 2024:** `bare_trait_objects` and `ellipsis_inclusive_range_patterns` are hard errors since 2021; what still does something is `elided_lifetimes_in_paths` (requires `Formatter<'_>`), `unused_extern_crates`, and `explicit_outlives_requirements`.
- **Priorities:** Cargo's reference says lower priorities "show up first on the command-line to tools like rustc" and are overridden by higher ones; the order inside the TOML table is ignored. Dropping `priority = -1` from the rustc groups failed clippy with `error: lint group 'let_underscore' has the same priority (0) as a lint`, pointing at `unsafe_code`, which isn't in that group (`clippy::lint_groups_priority`, deny by default). https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section
- **`redundant_pub_crate` vs `unreachable_pub` (tested):** in a private module, `pub(crate) fn` triggers `redundant_pub_crate` (use `pub`), and `pub fn` triggers `unreachable_pub` (use `pub(crate)`). No visibility satisfies both. `redundant_pub_crate` has been in nursery since 2020. Keeping `unreachable_pub` means `pub` always marks the real exported API, which for wasm-bindgen crates is the JS API. https://github.com/rust-lang/rust-clippy/blob/master/clippy_lints/src/redundant_pub_crate.rs
- **Overlap between rustc and clippy is common:** e.g. rustc's `unused_lifetimes`, `unit_bindings`, and `single_use_lifetimes` fire on the same code as clippy's `extra_unused_lifetimes`, `let_unit_value`, and `needless_lifetimes`/`elidable_lifetime_names` (tested). No guide recommends tracking this; it only means two warnings for one mistake.
- **Clippy's guidance** (https://doc.rust-lang.org/clippy/usage.html): pedantic "may have some intentional false positives", so "you can expect to sprinkle multiple `#[allow(..)]`s" (Clippy lints itself with the whole group). For restriction, "you shouldn't enable the whole lint group, but cherry-pick lints from this group. Some lints in this group will even contradict other Clippy lints!" Clippy "is meant to be used with a generous sprinkling of `#[allow(..)]`s", and "if you disagree with a lint, don't feel bad disabling them for parts of your code or the whole project."
- Other write-ups (e.g. https://rustprojectprimer.com/checks/lints.html) mix rustc and clippy lints in one `[lints]` table without discussing overlap.

### Nix toolchain

- At the repo's locked nixpkgs (2026-10-05): wasm-bindgen-cli 0.2.127, rustc 1.98.1, binaryen 132. nixpkgs head (2026-10-08) has wasm-bindgen-cli 0.2.129. Use the flake's pinned input, not the local registry's older `nixpkgs`.
- nixpkgs has versioned attrs (`wasm-bindgen-cli_0_2_127`, `_126`, `_125`, …); plain `wasm-bindgen-cli` is an alias for the newest. Each is a `buildWasmBindgenCli` call with two hashes, which is the fallback for a version nixpkgs lacks. crane's trunk example hard-codes the attr ("must match the one from Cargo.lock"); reading it from `Cargo.lock` is the same idea without a second copy.
- **Mismatch behaviour:** CLI 0.2.126 on 0.2.127 crate output exited 0 with no warning (so silent drift is possible). CLI 0.2.121 on 0.2.129 errored ("schema versions must exactly match"). With the `Cargo.lock`-derived selection, a lockfile on 0.2.128 (absent from nixpkgs) fails at eval with `Did you mean one of wasm-bindgen-cli_0_2_108, ...?`.
- **Toolchain options:**
  - **nixpkgs rustc/cargo:** ships wasm32 std, but needs `pkgs.lld` (nixpkgs patches out `rust-lld`) and `RUST_SRC_PATH` for rust-analyzer, and lags stable.
  - **rust-overlay (chosen):** `rust-bin.fromRustupToolchainFile` needs no hash; pinned by `flake.lock`. Officially targets x86_64-linux; works on aarch64-linux. Very active. https://github.com/oxalica/rust-overlay
  - **fenix:** `fromToolchainFile` needs the manifest `sha256` in pure eval, so the version lives in two places. No aarch64-linux cache.
  - **rustup in the devShell:** impure, not pinned by `flake.lock`, CI re-downloads each run.
- **rust-overlay tested** (nixpkgs head, rust-overlay 2026-10-12, `channel = "stable"`): rustc/cargo/clippy/rust-analyzer 1.99.0, wasm-bindgen 0.2.129. No `lld` on `PATH`; the release wasm build still worked (rust-lld is bundled in the sysroot).
- The current devShell already fails to evaluate on Darwin because of `chromium`; the Rust/wasm attrs evaluate fine on aarch64-darwin (not run on a real Mac).
- wasm-bindgen-cli comes from cache.nixos.org; the rust-overlay toolchain doesn't (see CI and deploy speed).
- nix-direnv's `use flake` watches only `flake.nix`, `flake.lock`, and `devshell.toml`, so `Cargo.lock` needs an explicit `watch_file`.

### wasm-bindgen output and Node

- Targets: `bundler`, `web`, `nodejs`, `no-modules`, `deno`, `experimental-nodejs-module`, `module`. `bundler` emits `import * as wasm from "./x_bg.wasm"`, which Vite 8.1+ handles natively; `web` uses `fetch`, which fails under Node at build time; `module` uses `import source`, which Vite/Rolldown don't support.
- TypeScript output is on by default and keeps doc comments, e.g. `export function add(a: number, b: number): number;`.
- `--weak-refs` and `--reference-types` are gone (detected automatically). `--remove-name-section --remove-producers-section` saved about 100 B on a toy crate.
- Node 22.19+/24.5+ import bundler output directly with an "Importing WebAssembly module instances is an experimental feature" warning. Vite's server build and dev module runner inline their own glue, so they don't depend on Node's support.

### Rust wasm target, size, and panics

- Default target features (1.99): bulk-memory, multivalue, mutable-globals, nontrapping-fptoint, reference-types, sign-ext. Current browsers support them; `wasm32v1-none` / `-Ctarget-cpu=mvp` aren't needed.
- **wasm-opt** must be told the features: plain `wasm-opt -Oz` fails with "memory.copy operations require bulk memory operations". Use `--enable-bulk-memory --enable-nontrapping-float-to-int --enable-reference-types --enable-multivalue --enable-sign-ext --enable-mutable-globals`.
- wasm-bindgen does no optimization itself; it only strips unneeded bindings.
- **`strip = true`**, measured on the smoke-style crate (`greet`, `parse_byte`), Rust 1.99, wasm-bindgen 0.2.129: cargo output 34,787 B vs 1,678,160 B without; after wasm-bindgen 19,242 B vs 28,093 B; gzipped 8,461 B vs 10,555 B. It doesn't break wasm-bindgen's custom sections.
- **Sizes**, toy crate (`add`, `greet`), `opt-level = "s"`, LTO, `codegen-units = 1`, strip: cargo 30,130 B → wasm-bindgen 15,562 B → `wasm-opt -Oz` 13,830 B (about 11% smaller).
- **opt-level**, larger demo crate, after wasm-bindgen: `"s"` 20,981 B, `"z"` 22,378 B, `3` 19,393 B. Start with `"s"` and re-measure with real code.
- **Panics:** `panic = "abort"` is the wasm32-unknown-unknown default, so leaving it out of the profile keeps native tests normal. A panic is a trap with no message; `console_error_panic_hook` still works but is archived. `panic = "unwind"` (JS `PanicError` exceptions) needs nightly + `-Zbuild-std`. The plan avoids panics instead: strict lints and `Result<_, JsError>`.

### Testing options

- **Native tests can't cross into JS.** A native test of `parse_byte("300")`, whose error path calls `JsError::new`, panicked with `cannot call wasm-bindgen imported functions on non-wasm targets`. The same applies to any `JsValue`, `JsError`, `js_sys`, or `web_sys` value. Hence the thin-wrapper pattern.
- **Host target is implicit:** with only `wasm32-unknown-unknown` in `targets`, native `cargo test` still compiled and ran.
- **wasm32 vs. native clippy** (`cargo clippy --all-targets --target wasm32-unknown-unknown`) reported a `dbg!` inside `#[cfg(test)] mod tests` and a missing `# Errors` section, the same as the native run (Rust 1.99).
- **wasm-bindgen-test 0.3.79** with `runner = "wasm-bindgen-test-runner"` ran under Node. `cargo test -q` fails (runner rejects `--quiet`), plain `#[test]` items don't run on wasm, and browsers need `CHROMEDRIVER`. Skipped: the Playwright test covers the boundary in three engines with no extra runner.

### Vite 8

Changelog: https://raw.githubusercontent.com/vitejs/vite/main/packages/vite/CHANGELOG.md. Current Vite is 8.3.4 (2026-10-08).

- **8.1.0** (2026-06-23): direct `.wasm` imports (WASM ESM Integration), https://github.com/vitejs/vite/pull/21779. Vite's own `vite:wasm-helper` generates the glue with top-level await: client uses `fetch` + `instantiateStreaming`, server uses `readFile(new URL(file, import.meta.url))` or an inlined `data:` URL.
- **8.3.3 / 8.2.4 / 8.1.6** (2026-10-06): fix GHSA-vfpm-58rq-9qcg, `?vite-wasm-instance` requests bypassed `server.fs` checks. The repo is on 8.2.2 with `host: true` and `allowedHosts: true`. astro 7.3.8 requires vite ^8.3.1.
- **Source-phase imports** (`import source`) are not supported by Vite/Rolldown, and Chromium 154 supports neither them nor native wasm ESM imports, so Vite's glue is required.
- **Environment API:** `configEnvironment(name, config, env)` is the documented place for per-environment settings. https://vite.dev/guide/api-environment-plugins
- Running an external build synchronously in `resolveId` blocks every resolve, runs per environment, and fires repeatedly; avoid it.
- Rerunning `wasm-bindgen` into a watched directory during `astro dev` worked: the next request served the new output, server-rendered and client-side.

### Astro 7

- **Docs:** only `WebAssembly.instantiateStreaming(fetch('/example.wasm'))`. No wasm integration.
- **Prerender bug:** in `dist/core/build/static-build.js`, static output runs `ssrMoveAssets()` before `generatePages()`. The prerender chunk still points at `new URL("../_astro/hello_bg.<hash>.wasm", import.meta.url)` inside `dist/.prerender/`, which has been moved to `dist/_astro/`:
  ```
  [ERROR] Error: ENOENT: no such file or directory, open '.../site/dist/.prerender/_astro/hello_bg.CN7ZGbTL.wasm'
  ```
  Reproduces with both `build.format` values, on astro 7.3.5 and 7.3.8. A web search found no existing issue (inconclusive).
- **`vite.environments.prerender.build.*` is silently dropped** by `core/build/vite-build-config.js`. A plugin's `configEnvironment` does take effect.
- **Fix (tested):** `configEnvironment` returning `{ build: { assetsInlineLimit: (f) => f.endsWith('.wasm') || undefined } }` for non-client environments. Frontmatter, Svelte island SSR, and nested pages all rendered; in `astro preview` the client loaded `/_astro/*.wasm` as `application/wasm`. Inlining costs nothing in output since `dist/.prerender` is deleted after generation. `astro check` flags (TS 7030) a `configEnvironment` that doesn't return on every path; the prototype added an explicit `return undefined`. The plan's single-ternary form returns on every path, but hasn't been run through `astro check` yet.
- **Alternatives:** externalizing the package for server environments worked but would probably break dev invalidation; a global `assetsInlineLimit` also inlines into client JS; `vite.ssr.external` is ignored for prerender.
- `astro check` runs integrations' `astro:config:setup` with `command = "sync"`, and does not type-check `.svelte` files (only `svelte-check` caught a deliberate error).
- Astro 7's `astro dev` backgrounds itself by default; `ASTRO_DEV_BACKGROUND=1` keeps it in the foreground, like the existing `ASTRO_PREVIEW_BACKGROUND=1`.

### Import styles compared

All tested for build, dev, and types.

| Style | Result |
| --- | --- |
| Relative path to `pkg/x.js` | Works; types from wasm-bindgen's `.d.ts`. Long paths. |
| **package.json `"imports"`** (chosen) | Works in Vite build and dev; `astro check` and `svelte-check` resolve types; no plugin, no `.d.ts`; resolved lazily so `pnpm install` doesn't need `pkg/`. Wildcard `"#rust/*"` tested. |
| pnpm workspace (`packages: [crates/*/pkg]`) | Works, but `pkg/` must exist before `pnpm install`. |
| Virtual `rust:x` module or `.rs` imports | Needs a plugin plus types via tsconfig `paths` or `allowArbitraryExtensions`. More machinery for no gain. |

### Existing plugins

`vite-plugin-wasm` and `vite-plugin-top-level-await` are redundant on Vite 8.1+. `@wasm-tool/rollup-plugin-rust` 3.1.6 inlined the wasm as base64 in the JS bundle and gave no types. `vite-plugin-rsw` and `vite-plugin-wasm-pack` are stale. None addresses the Astro prerender bug.

### Prior design (from an earlier chat)

A 58-line Vite plugin, tested 2026-10-10 on Astro 7.3.8: on a `.rs` import, `resolveId` ran `wasm-pack build` synchronously and wrote `lib.d.rs.ts` files for types; `configEnvironment` inlined `.wasm` for non-client environments; `server.watcher` rebuilt on Rust changes. Results: Rust edits reloaded in about 0.6–0.8 s, compile errors kept the last good build, Svelte `client:load` hydrated without warnings. Caveats: `execFileSync` paused the dev server while cargo compiled, and `astro build` had to run before `astro check` on a fresh checkout. Its usage rules are kept in step 4.

This plan keeps native Vite wasm imports, the `configEnvironment` inlining, and watcher-driven rebuilds. It replaces wasm-pack with `cargo` + `wasm-bindgen` from Nix, the `.rs` resolver with the `#rust/*` alias, and the synchronous build with async builds in Astro hooks. Building on `astro:config:setup` removes the "build before check" problem.

### CI and deploy speed

#### Current state (runs on 2026-10-11)

- CI runs only on `pull_request`, on `ubuntu-24.04-arm`. Latest run, 67 s: install Nix 3 s, **`pnpm install` 30 s**, cache images 2 s, **tests 24 s**.
- Nothing caches the Nix store; every run downloads the devShell from cache.nixos.org. The only `actions/cache` step caches Astro's optimized images.
- Deploys use Cloudflare Pages' git integration (project `sullvn`), building in Cloudflare's image.

#### Measured Rust cost (4 cores, aarch64, same as the GitHub ARM runner)

- Toolchain: about 150 MB of xz downloads (rustc 66 MB, two std libs 54 MB, the rest small), about 1.2 GB unpacked. rust-overlay builds it on the runner from these tarballs; rebuilding every toolchain derivation one at a time took **17 s**, an upper bound.
- Cargo, smoke-style crate, `CI=1`, fresh `target/`: clippy 4.8 s (native) / 5.6 s (wasm32), `cargo test` 4.8 s, release wasm build 5.0 s, `wasm-bindgen` under 0.1 s. Cold total about 21 s, `target/` 352 MB. With cached dependencies only our own crates rebuild (about 1–2 s).
- **Sharing between commands:** without an explicit target, a wasm32 command rebuilds all 19 units including proc-macros. With `--target host-tuple` on native commands, proc-macros compiled once and each extra configuration rebuilt only 6–7 units; `target/` was 223 MB vs 351 MB. The release wasm build always rebuilds everything (20 units).

#### GitHub Actions caching

- **Scoping:** a run restores caches from its own branch, its base branch, or the default branch. A `pull_request` run's cache "can only be restored by re-runs of the pull request", so with PR-only CI no cache is shared across PRs, including `astro-images`. A `push` run on `main` fixes it. https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching
- **Limits:** entries unused for 7 days are evicted; 10 GB per repo.
- **Swatinem/rust-cache** v2.9.2 (2026-08-06): caches `~/.cargo` and dependency artifacts in `./target` (not workspace crates), prunes stale artifacts, falls back to caches from older lockfiles. `cmd-format` supports Nix.
- **sccache** with the GitHub backend: per-object and rate-limited; not worth it for a small workspace.

#### Nix store caching

- **magic-nix-cache-action** v15 (2026-09-09), active again after a reversed end-of-life. Free; stores only paths not on cache.nixos.org (here, the rust-overlay toolchain).
- **cache-nix-action** would archive all of `/nix`, including Chromium and the Playwright browsers; likely slower than cache.nixos.org. **cachix** needs a token secret; **FlakeHub Cache** is paid.

#### Cloudflare builds

- The Pages build image (v3, x86_64) has **no Rust, cargo, rustup, or wasm32 target**, and its build cache only covers fixed directories (`.pnpm-store`, `node_modules/.astro`, …), so `~/.cargo` and `target/` can't be cached. Installing Rust there would download about 165 MB per build, and `channel = "stable"` would drift from `flake.lock`. Workers Builds has the same limits. Hence building in GitHub Actions.
- Cloudflare's Pages docs now say "Start new projects with Workers"; Pages isn't formally deprecated.

#### Deploying from GitHub Actions

- `cloudflare/wrangler-action` v4.1.3 (2026-09-24), Wrangler v4.
- **Wrangler from Nix instead:** nixpkgs has `wrangler` 4.148.0 for aarch64-linux on cache.nixos.org, but it is a 171 MB download (664 MB unpacked), paid on every CI run without a Nix store cache. The npm package wrangler-action installs is about 170 MB unpacked, most of it `workerd`.
- A git-connected Pages project can't switch to Direct Upload, but turning off automatic deployments (Branch control) lets wrangler deploy to it. Project, domain, and URLs stay. https://developers.cloudflare.com/pages/configuration/git-integration/
- `wrangler pages deploy dist --project-name=sullvn --branch=<name>` gives `<branch>.sullvn.pages.dev`. Needs `CLOUDFLARE_API_TOKEN` (Account > Cloudflare Pages > Edit), `CLOUDFLARE_ACCOUNT_ID`, and `deployments: write` + `gitHubToken` for a GitHub Deployment.
- **Workers equivalent:** `wrangler.jsonc` with `"assets": {"directory": "./dist"}`, `wrangler deploy`, and Workers Previews (`wrangler preview`) at `<preview>-<worker>.<subdomain>.workers.dev`. https://developers.cloudflare.com/workers/static-assets/migration-guides/migrate-from-pages/

### Hygiene tools (not adopted yet)

- **cargo-deny** 0.20.2: a tested `deny.toml` passed with `[graph] targets` for wasm32 + linux, `unmaintained = "all"`, `yanked = "deny"`, licenses `MIT`, `Apache-2.0`, `Unicode-3.0`, `multiple-versions = "warn"`, `wildcards = "deny"`.
- **cargo-shear** 1.14.0: unused deps on stable; preferred over regex-based cargo-machete.

### Tested in scratch projects

- **rust-overlay** with `channel = "stable"`: Rust 1.99.0, release wasm build, `wasm-bindgen` 0.2.129.
- **Integration prototype** (Astro 7.3.5, Vite 8.2.2, wasm-bindgen 0.2.127):
  - `#rust/*` wildcard imports resolve; `const x: string = add(1, 2)` fails `astro check` with TS2322.
  - `astro check` logged `config:setup command=sync` and `built crates (debug) in 8408ms` before type-checking.
  - `astro build` rendered the thrown `JsError` message (`number too large to fit in target type`) into the HTML and emitted `dist/_astro/index_bg.<hash>.wasm`.
  - In `astro dev`, a Rust edit was live in about 2 s (`built crates (debug) in 135ms`). A syntax error logged cargo's error, the page kept serving, and it recovered on the next save.
- **`--message-format=json-render-diagnostics`** (Rust 1.99): stdout held only `compiler-artifact`, `build-script-executed`, and `build-finished` lines, with the `.wasm` path in `filenames`; a syntax error printed cargo's usual rendered error on stderr, exit 101.
- Svelte islands with `client:load` and `client:only` worked in build and preview (headless Chromium).

### Not yet verified

- That an already-open tab updates during dev (only fresh requests checked).
- That Firefox and WebKit run the client wasm (covered by the new Playwright test).
- That the flake change works in CI (the `mkRustBin` toolchain was only evaluated, not built).
- The integration's changes since the prototype, which only exist on paper: picking crates with `cargo metadata`, `--profile wasm` with `--target wasm32-unknown-unknown`, the `building`/`dirty` coalescing, and the single-ternary `configEnvironment` passing `astro check`.
- Actual CI timings with Rust on the GitHub runner, and how much magic-nix-cache would save.
- The current Pages dashboard build command and image version.
- That the `dist/` built for Playwright is identical to a production build (no difference in build env is visible).
- Server-adapter output (not needed for this static site).
- Whether an Astro issue for the prerender bug already exists.
