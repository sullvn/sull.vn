# Rust → wasm for the Astro site

## Goal

The `rust-integration` branch prepares the site for Rust → wasm code. It should end with:

1. A Rust workspace with config and lints set up to be as modern and strict as possible.
2. A working Rust → wasm integration for Astro.
3. A test crate and a site page that exercise the full pipeline.

Everything should be as simple, ergonomic, understandable, modern, and strict as possible.

Decisions so far:

- Test page lives at `/rust-wasm-test`.
- Use the latest stable Rust, pinned by `flake.lock`.
- Update the project flake as needed.
- No supply-chain checks (`cargo-deny`, `cargo-shear`) yet.
- Keep CI and deploys fast. Cloudflare stops building the site; GitHub Actions builds, tests, and deploys `dist/` with wrangler.
- No two tools doing the same job (e.g. no second Nix cache unless measurements justify it).

## Implementation steps (short)

1. **Update dependencies:** `nix flake update`, bump astro to 7.3.8 (brings Vite's security fix).
2. **CI and deploys:** run CI on pushes to `main` too (so caches are shared), deploy previews and production from GitHub Actions with `cloudflare/wrangler-action`, turn off Cloudflare's own builds.
3. **Rust workspace:** rust-overlay toolchain from `rust-toolchain.toml` (`channel = "stable"`), `wasm-bindgen-cli` matched to `Cargo.lock`, workspace `Cargo.toml` with strict lints, rustfmt/clippy config, `test:cargo` script with `--target host-tuple`, `Swatinem/rust-cache` in CI.
4. **Astro integration:** `integrations/rust.ts` builds crates with `cargo` + `wasm-bindgen`, fixes Astro's prerender wasm bug, rebuilds on Rust edits in dev; `#rust/*` import alias; `svelte-check`.
5. **Test crate and page:** `crates/smoke`, `/rust-wasm-test` page using it at build time, in a client script, and in a Svelte island; Playwright test in Chromium, Firefox, and WebKit.

Optional, after measuring: `magic-nix-cache-action` for the Rust toolchain, `CARGO_PROFILE_DEV_DEBUG=0` in CI, migrating from Pages to Workers static assets.

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
    if: github.event_name == 'push' || github.event.pull_request.head.repo.full_name == github.repository
    uses: cloudflare/wrangler-action@<sha> # v4.1.3
    with:
      apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
      accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
      gitHubToken: ${{ secrets.GITHUB_TOKEN }}
      command: pages deploy dist --project-name=sullvn --branch=${{ github.head_ref || github.ref_name }}
  ```

  - `main` deploys production; any other branch gets a `<branch>.sullvn.pages.dev` preview.
  - The `if` skips fork and Dependabot PRs, which can't read secrets.
  - Deploys only happen when tests pass.

#### Manual steps (owner)

1. Create a Cloudflare API token with **Account > Cloudflare Pages > Edit**.
2. Add `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID` as repo secrets.
3. After the new workflow has deployed successfully once, turn off Cloudflare's builds in Pages > Settings > Build > Branch control: untick automatic production deployments, set preview branches to None. The `sullvn` project, its domain, and URLs stay the same.

Trade-off: the Cloudflare bot's PR comment and check run become a GitHub Deployment link.

### 3. Rust workspace

#### `flake.nix`

Add the rust-overlay input, with its nixpkgs following ours, and apply its overlay. Add to the devShell:

```nix
let
  # wasm-bindgen-cli must match the wasm-bindgen crate in Cargo.lock exactly.
  # Evaluation fails if nixpkgs has no package for that version.
  wasmBindgen = pkgs.lib.findFirst (p: p.name == "wasm-bindgen") (throw "wasm-bindgen is not in Cargo.lock") (
    builtins.fromTOML (builtins.readFile ./Cargo.lock)
  ).package;
  wasm-bindgen-cli =
    pkgs."wasm-bindgen-cli_${builtins.replaceStrings [ "." ] [ "_" ] wasmBindgen.version}";
in
# ...
packages = [
  # ...existing packages
  (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
  wasm-bindgen-cli
  pkgs.binaryen # wasm-opt
];
```

`Cargo.lock` must be committed (and tracked by git) for the flake to read it.

**Update flow:** `nix flake update` brings the latest stable Rust (rust-overlay) and the latest `wasm-bindgen-cli` (nixpkgs). Then set the `=` pin on wasm-bindgen in `Cargo.toml` to the nixpkgs version. If they disagree, `nix develop` fails with an error listing the versions nixpkgs has.

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
- `[workspace.package]`: `edition = "2024"`, `publish = false`. No `rust-version`.
- `[workspace.dependencies]`: `wasm-bindgen = "=0.2.129"`.
- `[workspace.lints.rust]`: `unsafe_code = "forbid"`, `missing_docs`, `missing_debug_implementations`, `unreachable_pub`, `unused_qualifications`, `rust_2018_idioms`, `let_underscore_drop`, `meta_variable_misuse`, `redundant_lifetimes`, `single_use_lifetimes`, `trivial_casts`, `trivial_numeric_casts`, `unit_bindings`, `unnameable_types`, `unused_import_braces`, `unused_lifetimes`, `unused_macro_rules`.
- `[workspace.lints.clippy]`:
  - Groups: `pedantic`, `nursery`, `cargo` (each `priority = -1`).
  - Restriction lints: `allow_attributes`, `allow_attributes_without_reason`, `as_conversions`, `clone_on_ref_ptr`, `dbg_macro`, `empty_structs_with_brackets`, `expect_used`, `get_unwrap`, `if_then_some_else_none`, `indexing_slicing`, `missing_assert_message`, `mod_module_files`, `panic`, `print_stderr`, `print_stdout`, `redundant_type_annotations`, `str_to_string`, `string_slice`, `tests_outside_test_module`, `todo`, `try_err`, `undocumented_unsafe_blocks`, `unimplemented`, `unused_result_ok`, `unwrap_used`.
  - Turned off, each with a comment: `missing_const_for_fn` (wasm-bindgen rejects `const fn`), `multiple_crate_versions` (wasm-bindgen's own dependencies trigger it).
- `[profile.release]`: `opt-level = "s"`, `lto = true`, `codegen-units = 1`, `strip = true`.

Lints are warnings while working; `CARGO_BUILD_WARNINGS=deny` turns them into errors in `pnpm test` and CI.

#### Other files

- `rustfmt.toml`: `style_edition = "2024"`, `use_field_init_shorthand = true`, `use_try_shorthand = true` (stable options only).
- `clippy.toml`: `allow-unwrap-in-tests`, `allow-expect-in-tests`, `allow-panic-in-tests`, `allow-indexing-slicing-in-tests`, `avoid-breaking-exported-api = false`.
- `.gitignore`: `target/`, `crates/*/pkg/`.
- `.envrc`: `watch_file Cargo.lock`.
- No `.cargo/config.toml`. In particular, no `[build] target = "wasm32-unknown-unknown"`: it would make plain `cargo test` build for wasm and stop native tests running. The integration passes `--target` explicitly.

#### `package.json` scripts

- `test:cargo`: `cargo fmt --check`, clippy (native `--all-targets` and `--target wasm32-unknown-unknown`), `cargo test`, `cargo doc --no-deps`, all with `CARGO_BUILD_WARNINGS=deny`. Added to `test`.
  - The native clippy, test, and doc commands pass `--target host-tuple`. Without it, the wasm32 commands recompile every dependency including wasm-bindgen's proc-macros; with it, proc-macros and build scripts compile once and are shared by every command except the release build. `target/` shrinks from 351 MB to 223 MB, so the CI cache does too.
- `format`: also runs `cargo fmt`.

#### CI caching

Add `Swatinem/rust-cache` (v2.9.2, pinned by SHA) to `checks.yaml`, after `Install Nix`:

```yaml
- name: Cache Rust
  uses: Swatinem/rust-cache@<sha> # v2.9.2
  with:
    cmd-format: nix develop --no-update-lock-file -c {0}
    save-if: ${{ github.ref == 'refs/heads/main' }}
    cache-bin: false
```

- Caches `~/.cargo` and compiled dependencies in `target/`, keyed on the Rust version, `Cargo.lock`, `Cargo.toml`, `rust-toolchain.toml`, and `CARGO*`/`RUST*` env vars.
- `save-if` means only `main` saves; PRs restore `main`'s cache and don't fill the 10 GB limit.
- Cargo 1.99 already disables incremental builds when `CI` is set, so no `CARGO_INCREMENTAL=0`.

Expected CI time: about 75–90 s (vs. about 67 s today, and about 105–110 s with Rust and no caching). See research for the breakdown.

### 4. Astro integration

#### `package.json`

```jsonc
"imports": { "#rust/*": "./crates/*/pkg/index.js" }   // import { add } from '#rust/smoke'
```

#### `integrations/rust.ts`

About 60 lines, registered in `astro.config.ts`.

- **Build on `astro:config:setup`.** Runs for `dev`, `build`, and `sync` (which `astro check` runs first), so the generated package and its types exist before anything reads them. Skipped for `preview`.
  1. `cargo metadata` lists the cdylib crates.
  2. `cargo build --target wasm32-unknown-unknown` builds them (`--release` for `astro build`).
  3. `wasm-bindgen --target bundler --out-name index` writes each crate's `pkg/`.
  4. Release builds run `wasm-opt -Os` with the six default Rust wasm features enabled.
- **Prerender fix.** A `configEnvironment` plugin inlines `.wasm` for every environment except `client`. File the Astro bug upstream and link it from a comment.
- **Dev rebuilds on `astro:server:setup`.** Watch `crates/*/src` and `Cargo.toml`, rebuild without blocking the dev server, and coalesce saves made during a build into one more build. Log cargo errors; the page keeps the last good build. Vite picks up the regenerated `pkg/` and reloads.
- **Config:** add `**/target/**` to `vite.server.watch.ignored`.

Types come from wasm-bindgen's generated `index.d.ts`: no `.rs` imports, no hand-written type files, no wasm-pack, no wasm plugins.

#### Svelte type checking

Add `svelte-check` as a `test:svelte` script, since `astro check` skips `.svelte` files.

### 5. Test crate, page, and end-to-end test

- **`crates/smoke`**, one export per kind of value that crosses the boundary:
  - `add`: numbers
  - `greet`: strings
  - `sum(&[f64])`: typed arrays
  - `parse_byte`: `Result<u8, JsError>`, thrown as an exception in JS
  - `Counter`: a class with methods, for state
- **Thin wrappers.** Logic lives in plain Rust functions; `#[wasm_bindgen]` exports only convert at the boundary. Native tests can't call anything that creates a `JsValue` or `JsError`, so the logic must be reachable without them:

  ```rust
  /// Parses a decimal byte.
  #[wasm_bindgen]
  pub fn parse_byte(s: &str) -> Result<u8, JsError> {
      parse(s).map_err(|e| JsError::new(&e.to_string()))
  }

  fn parse(s: &str) -> Result<u8, ParseIntError> {
      s.parse()
  }
  ```

- **Rust tests**, in a `#[cfg(test)] mod tests` at the bottom of each file (enforced by `tests_outside_test_module`), plus doctests. They run natively via `cargo test`, locally or through `pnpm test:cargo` in CI.
- **Who tests what:**
  - `cargo test` (native): the logic. Fast, no browser or Node.
  - Playwright: the boundary, meaning the real build crossing into JS in three engines, including errors arriving as JS exceptions and class state.
  - No `wasm-bindgen-test` for now; add it if crates later get heavy `web-sys` / `js-sys` code that needs fast Rust-level boundary tests.

### Optional: explore after measuring

Only once real CI timings exist, and only if they justify the extra moving part:

- **`DeterminateSystems/magic-nix-cache-action`** (v15). Caches Nix store paths that aren't on cache.nixos.org, which here is just the rust-overlay toolchain. It doesn't overlap with anything existing: `cachix/install-nix-action` only installs Nix, and cache.nixos.org (Nix's built-in binary cache) keeps serving everything nixpkgs builds. Estimated saving 10–15 s, unverified. Measure the toolchain setup time on the first runs; add it only if that's a meaningful share.
- **`CARGO_PROFILE_DEV_DEBUG=0`** as a CI env var: smaller `target/`, faster linking. rust-cache folds it into its key automatically.
- **Workers static assets** instead of Pages (`wrangler deploy` with an `assets` directory, `wrangler preview` for PRs). Cloudflare now steers new projects to Workers, but it's a separate migration (URLs move to workers.dev) and not needed for Rust.
- **`src/pages/rust-wasm-test.astro`** uses the crate three ways:
  - in frontmatter, baked into the HTML at build time
  - in a client `<script>`
  - in a Svelte island, `src/components/RustCounter.svelte` with `client:load`
  - It also shows the thrown error.
- **`tests/rust-wasm.spec.ts`** checks the build-time HTML, clicks through the client script and the island, and checks the error message. Runs as three new Playwright projects (Chromium, Firefox, WebKit). The existing visual projects get `testMatch: 'visual.spec.ts'`.

## Research

Researched 2026-10-12, by four parallel research passes (Rust workspace, wasm toolchain, Vite/Astro integration, Nix) plus follow-up prototypes. Versions and behaviour were checked against primary sources and, where noted, by running real builds in scratch projects on this machine (aarch64-linux, same arch as CI).

### Ecosystem status

- **rustwasm org sunset:** announced 2025-07-21 by Alex Crichton, archived September 2025. https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org/
- **wasm-bindgen** moved to https://github.com/wasm-bindgen/wasm-bindgen (`rustwasm/wasm-bindgen` redirects). The org also holds `wasm-pack`, `walrus`, `weedle`, and two experimental repos with no releases, `js-bindgen` and `ts-gen`.
  - Latest **0.2.129** (2026-09-25). Recent: 0.2.128 (09-04), 0.2.127 (08-08), 0.2.126 (06-24), 0.2.125 (06-12), 0.2.123 (06-08), 0.2.122 (05-22), 0.2.121 (05-07).
  - Cadence is every 2–5 weeks. The 0.2.x series still has small breaking changes (0.2.122 and 0.2.123 had some).
  - Library MSRV is 1.81 as of 0.2.129.
  - Companion crates release in lockstep (all 2026-09-25): js-sys 0.3.106, web-sys 0.3.106, wasm-bindgen-futures 0.4.79, wasm-bindgen-test 0.3.79, wasm-bindgen-cli-support 0.2.129.
  - wasm-bindgen 0.2.129 pulls in `syn v3.0.9`, so duplicate syn 2/3 versions are likely as crates are added.
  - Releases: https://github.com/wasm-bindgen/wasm-bindgen/releases
- **wasm-pack** was transferred, not archived: https://github.com/wasm-bindgen/wasm-pack, README says "maintained by drager".
  - v0.15.0 (2026-05-15): wasm64, `--panic-unwind`, fixed npm installer. v0.14.0 (2026-01-20): any wasm target, `--profile`, `--split-linked-modules`. Before that v0.13.1 (2024-10-29). Last push 2026-08-12.
  - What it adds, from its source: reads the wasm-bindgen version from `Cargo.lock` (`src/lockfile.rs`); uses a `wasm-bindgen` on `PATH` only if `--version` matches exactly, otherwise downloads a prebuilt binary, then falls back to `cargo install` (`src/install/mod.rs`); `--mode no-install` disables downloads; downloads binaryen (master pins `version_130`) and runs `wasm-opt` on release builds, taking any `wasm-opt` on `PATH` without a version check (`src/wasm_opt.rs`); writes `package.json` and README for npm.
  - In a Nix setup the runtime downloads are a downside.
- **gloo** moved to ranile/gloo (not archived); 0.12.0 (2026-03-25), first release since 2023. crates.io metadata still points at the old rustwasm URL.
- **console_error_panic_hook** is archived: last release 0.1.7 (2021-10-11).
- **cargo-component** is dormant: last release v0.21.1 (April 2025), last push 2025-07-14.
- **Component model / jco** is active: `rolldown-plugin-jco` v0.2.0 (2026-10-08), preview2-shim 0.28 and preview3-shim 0.9 (2026-10-06). For browser + Node it means WIT files, WASI shims, and a transpile step; DOM access is awkward. Not tested; not the simpler option today.
- **Trunk** v0.22.0-rc.2 (2026-10-06) is for Rust-only apps, not this use case.

### Rust toolchain and language

- Current stable: **1.99.0** (2026-10-01; manifest says `1.99.0 (b940084d7 2026-09-28)`). 2026 point releases: 1.98.1 (Sep 3), 1.97.1, 1.96.1. https://blog.rust-lang.org/releases/, https://static.rust-lang.org/dist/channel-rust-stable.toml
- Edition **2024** (shipped in 1.85) is still the latest. No 2027 edition is announced: the edition guide has no 2027 chapter, none of the 81 goals in the 2026 Project Goals slate is an edition goal (https://goals.rust-lang.org/2026/goals.html), and an internals thread still debates 2027 vs 2028 (https://internals.rust-lang.org/t/we-have-only-two-years-to-prepare-the-rust-2027-edition/22494).
- Recent changes from https://raw.githubusercontent.com/rust-lang/rust/stable/RELEASES.md:
  - 1.97 stabilized Cargo `build.warnings` (`CARGO_BUILD_WARNINGS=deny`).
  - 1.98: "UNSAFE_CODE lint now emitted consistently for all unsafe attributes" (re-tested against wasm-bindgen; still fine).
  - 1.99 added a built-in `debug` profile and turns off incremental builds when the `CI` env var is set.
- **Resolver 3** is the edition-2024 default but a virtual workspace has no package edition, so set it explicitly. It sets `incompatible-rust-versions = "fallback"`: prefers dependency versions whose `rust-version` fits, never errors. Clippy also uses `rust-version` as its MSRV. Without `rust-version`, resolution uses the toolchain in use. https://doc.rust-lang.org/cargo/reference/resolver.html
- **`CARGO_BUILD_WARNINGS=deny`** beats `RUSTFLAGS=-Dwarnings`: RUSTFLAGS invalidates the build cache, applies to dependencies' build scripts, and overrides config rustflags. Tested: fails `cargo clippy` with "warnings are denied by `build.warnings` configuration" (exit 101), and fails `cargo doc` on a broken intra-doc link, so `RUSTDOCFLAGS=-D warnings` isn't needed. Keep it out of `.cargo/config.toml` so local work isn't blocked. https://doc.rust-lang.org/cargo/reference/config.html#buildwarnings
- Cargo's own `[lints.cargo]` is still nightly-only. https://doc.rust-lang.org/cargo/reference/lints.html
- Lints are not inherited implicitly; each crate needs `[lints] workspace = true`.
- `#[expect]` and `reason = "..."` stable since 1.81. Pairing `clippy::allow_attributes` with `allow_attributes_without_reason` was tested: both fire on a bare `#[allow]`.
- **rustfmt** (https://github.com/rust-lang/rustfmt/blob/main/Configurations.md; stable rustfmt prints "unstable features are only available in nightly channel" for nightly ones):
  - Stable: `style_edition`, `edition`, `use_field_init_shorthand`, `use_try_shorthand`, `newline_style`, `max_width`, `reorder_imports`.
  - Nightly-only: `imports_granularity` (#4991), `group_imports` (#5083), `reorder_impl_items` (#3363), `wrap_comments` (#3347), `format_code_in_doc_comments` (#3348), `imports_layout` (#3361).
  - `style_edition` defaults to 2015 for bare `rustfmt`; `cargo fmt` passes the crate edition. Setting it keeps editors consistent.
- **clippy.toml:** unknown keys error, so all chosen keys are verified accepted by clippy 1.99. `check-private-items = true` made `missing_panics_doc` fire on `assert_eq!` inside tests; left off.
- **rust-toolchain.toml:** rustup accepts `channel = stable | beta | nightly | <major.minor> | <major.minor.patch>` plus `components`, `targets`, `profile`. Ranks below `cargo +x`, `RUSTUP_TOOLCHAIN`, and `rustup override`, above the default toolchain. https://rust-lang.github.io/rustup/overrides.html
  - Plain `"stable"` with rustup silently changes lints every 6 weeks; with rust-overlay, `flake.lock` pins what "stable" means, which removes that downside.
- **Workspace layout:** `Cargo.toml` at the repo root next to `package.json`, members in `crates/*`, `[workspace.package]` inheritance, `[workspace.dependencies]`. wasm crates need `crate-type = ["cdylib", "rlib"]`; `rlib` lets native tests and doctests work.

### Strict lints vs. wasm-bindgen

Tested on Rust 1.95 and 1.99, native and `--target wasm32-unknown-unknown`, exercising: free functions, a struct with an impl block (constructor, `&mut self` methods), getters/setters, `pub` fields, a C-style enum, an `extern "C"` import with `js_namespace`, `Result<_, JsError>`, `Option<String>`, `&JsValue`, `Vec<f64>` and `&[f64]`, `typescript_custom_section`, and `#[wasm_bindgen(start)]`.

- **`unsafe_code = "forbid"` works.** The expansion (`-Zunpretty=expanded`) contains 22 `unsafe` tokens plus `#[no_mangle]`, `#[export_name]`, and `#[link_section]`, yet nothing fires. A hand-written `unsafe {}` in the same crate does error ("usage of an unsafe block").
- **No noise from generated code** with `missing_docs`, `missing_debug_implementations`, `unreachable_pub`, `unused_qualifications`, pedantic, nursery, or the chosen restriction lints.
- **`clippy::missing_const_for_fn`** (nursery) suggests `const fn`, but then the macro fails: `error: can only #[wasm_bindgen] non-const functions`. Allow it workspace-wide; per-function `#[expect(clippy::missing_const_for_fn, reason = "...")]` also works.
- **`clippy::missing_inline_in_public_items`** fires on every export; pure noise.
- **`unused_crate_dependencies`** fires for dev-dependencies in lib-test and integration-test crates ("extern crate `demo` is unused in crate `web`"); left out.
- **`clippy::cargo`:** with `publish = false`, `cargo_common_metadata` didn't fire without description/repository/license (observed, not documented). `multiple_crate_versions` will fire because of syn 2/3.
- `needless_pass_by_value` (pedantic) correctly flags `Vec<f64>` params; `&[f64]` works with wasm-bindgen.
- `exhaustive_structs` works with `#[non_exhaustive]` on wasm-bindgen structs; left out for simplicity.
- Left out as noisy or poor fit: `std_instead_of_core`, `std_instead_of_alloc`, `alloc_instead_of_core` (only for `no_std`), `missing_docs_in_private_items`, `arithmetic_side_effects`, `min_ident_chars`, `pub_use`, `shadow_unrelated`, `single_char_lifetime_names`, `wildcard_enum_match_arm`.
- Enabling all of `clippy::restriction` is discouraged by clippy's own `blanket_clippy_restriction_lints`.
- `clippy::string_to_string` has been removed (merged into `implicit_clone`).
- The full chosen lint set passed with zero warnings on native and wasm32.

### Nix toolchain

Package versions at the repo's locked nixpkgs `aa48d347` (2026-10-05), via `nix eval`:

| Package | Locked rev | nixpkgs-unstable head `8edc0c72` (2026-10-08) | Upstream latest |
| --- | --- | --- | --- |
| rustc / cargo / clippy | 1.98.1 | 1.98.1 | 1.99.0 |
| rust-analyzer | 2026-09-28 | | |
| wasm-bindgen-cli | 0.2.127 | 0.2.129 | 0.2.129 |
| binaryen | 132 | 132 | version_133 (2026-09-21) |
| wasm-pack | 0.15.0 | 0.15.0 | 0.15.0 |
| lld | 21.1.8 | | |

- The `nixpkgs` in the local flake registry is an older 26.05 snapshot (rustc 1.95.0, wasm-bindgen-cli 0.2.121). Use the flake's pinned input, not `nix shell nixpkgs#...`.
- Versioned attrs at the locked rev: `wasm-bindgen-cli_0_2_93`, `_95`, `_99`, `_100`, `_104`, `_105`, `_106`, `_108`, `_114`, `_117`, `_118`, `_120`, `_121`, `_122`, `_125`, `_126`, `_127`. Plain `wasm-bindgen-cli` is an alias: `pkgs/top-level/aliases.nix:2735: wasm-bindgen-cli = wasm-bindgen-cli_0_2_127;`.
- Each versioned attr is a `buildWasmBindgenCli { src = fetchCrate {…; hash}; cargoDeps = rustPlatform.fetchCargoVendor {…; hash}; }` (`pkgs/by-name/wa/wasm-bindgen-cli_0_2_127/package.nix`). Writing one yourself, with two hashes, is the fallback for a version nixpkgs lacks.
- crane's trunk example hard-codes `wasm-bindgen-cli = pkgs.wasm-bindgen-cli_0_2_114; # must match the one from Cargo.lock` (https://github.com/ipetkov/crane/blob/master/examples/trunk/flake.nix). Reading it from `Cargo.lock` is the same idea without a second copy. Leptos and Bevy setups weren't checked.
- **Mismatch behaviour:**
  - `wasm-bindgen-cli_0_2_126` on 0.2.127 crate output exited 0 with no warning.
  - CLI 0.2.121 on crate 0.2.129 errored:
    ```
    rust Wasm file schema version: 0.2.129
       this binary schema version: 0.2.121
    Currently the bindgen format is unstable enough that these two schema versions must exactly match.
    ```
  - With the `Cargo.lock`-derived selection, setting the lockfile to 0.2.128 (absent from nixpkgs) fails at eval: `Did you mean one of wasm-bindgen-cli_0_2_108, wasm-bindgen-cli_0_2_118, ...?`
- **Toolchain options:**
  - **nixpkgs rustc/cargo:** ships std for `wasm32-unknown-unknown` and `wasm32v1-none` (`pkgs/development/compilers/rust/rustc.nix`). nixpkgs replaces `rust-lld` with `lld` in target specs ("This is not an appropriate default for Nixpkgs, where there is no rust-lld"), so without `pkgs.lld` the build fails with ``error: linker `lld` not found``. Pinned by `flake.lock`, cached on all platforms, but lags stable and can't pick an exact version or nightly. A tested devShell with it passed `cargo build`, `cargo clippy -- -D warnings`, and `nix flake check`. It needed `RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}"` for rust-analyzer.
  - **rust-overlay (chosen):** `rust-bin.fromRustupToolchainFile ./rust-toolchain.toml` needs no hash; manifests are vendored and pinned by `flake.lock`. README: "We target nixos-unstable … on x86_64-linux … Other platforms … not guaranteed." Very active (last commit 2026-10-12, 1.6k stars). https://github.com/oxalica/rust-overlay
  - **fenix:** `fromToolchainFile` needs the manifest `sha256` in pure eval ("required in pure evaluation mode"), so the version lives in two places. Cachix cache covers x86_64-darwin, aarch64-darwin, x86_64-linux, not aarch64-linux. Active (last commit 2026-10-12, 1.1k stars). https://github.com/nix-community/fenix
  - **rustup in the devShell:** impure, downloads into `~/.rustup`, not pinned by `flake.lock`, CI re-downloads each run.
- **rust-overlay tested** with nixpkgs head `8edc0c72` and rust-overlay `404b7a5` (2026-10-12), `channel = "stable"`:
  ```
  rustc 1.99.0 (b940084d7 2026-09-28)
  cargo 1.99.0 (5f94df478 2026-08-27)
  clippy 0.1.99 (b940084d7e 2026-09-28)
  rust-analyzer 1.99.0 (b940084 2026-09-28)
  wasm-bindgen 0.2.129
  wasm-opt version 132
  ```
  No `lld` or `rust-lld` on `PATH`; the release wasm build still worked (rust-lld is bundled in the toolchain's sysroot). `wasm-bindgen --target bundler` and `wasm-opt -Os` with the six feature flags passed.
- The current devShell already fails to evaluate on Darwin because of `chromium` ("not available on the requested hostPlatform"); all the Rust/wasm attrs evaluate fine on aarch64-darwin. Not run on a real Mac.
- Binary cache: wasm-bindgen-cli (aarch64-linux, aarch64-darwin) and darwin rustc are on cache.nixos.org; nothing builds from source. The nixpkgs Rust packages' own NARs are about 56 MB.
- The nixpkgs manual's Rust section only covers `wasm32-wasip1` via pkgsCross and says nothing about wasm-bindgen.
- **CI caching:** nothing needed now. `Swatinem/rust-cache` v2.9.2 supports Nix via `cmd-format: nix develop -c {0}` if target/ builds get slow. magic-nix-cache-action (v15) wouldn't help since nothing builds from source.
- nix-direnv may already reload on `Cargo.lock`; not verified, hence `watch_file Cargo.lock`.

### wasm-bindgen output targets

`--help` (0.2.121) lists `bundler`, `web`, `nodejs`, `no-modules`, `deno`, `experimental-nodejs-module`, `module`. Each writes `x.js`, `x.d.ts`, `x_bg.wasm`, `x_bg.wasm.d.ts`; `bundler` adds `x_bg.js`; `experimental-nodejs-module` adds a `package.json` with `"type":"module"`.

Key generated lines:

- **bundler:** `import * as wasm from "./x_bg.wasm";` and `import { __wbg_set_wasm } from "./x_bg.js";`. The `.wasm` imports `"./x_bg.js"` back.
- **web:** `export { initSync, __wbg_init as default };`; default init uses `new URL('x_bg.wasm', import.meta.url)` → `fetch` → `instantiateStreaming`.
- **module** (new in 0.2.102): `import source wasmModule from "./x_bg.wasm";` then `new WebAssembly.Instance(wasmModule, __wbg_get_imports())`.
- **experimental-nodejs-module:** `readFileSync(new URL('x_bg.wasm', import.meta.url))`, synchronous.
- **nodejs:** CommonJS, `require('fs').readFileSync`.

Running directly in Node:

| Output | Node 22.23.3 | Node 24.21.0 | Node 26.10.0 |
| --- | --- | --- | --- |
| bundler | works, ExperimentalWarning | works, ExperimentalWarning | works, ExperimentalWarning |
| module | SyntaxError on `import source` | works | works |
| web + `initSync({ module: readFileSync(...) })` | works | works | works |
| web + default `init()` | | | fails, "fetch failed" |
| experimental-nodejs-module | works | works | works |

The warning reads "Importing WebAssembly module instances is an experimental feature".

CLI flags:

- `--weak-refs` and `--reference-types` are gone: deprecated in 0.2.91 (runtime detection) and 0.2.96 (detected from target features).
- TypeScript output is on by default (`--no-typescript` disables it) and clean, keeping doc comments, e.g. `export function add(a: number, b: number): number;`.
- `--omit-default-module-path` drops the default `new URL(...)` path. `--split-linked-modules` is "recommended if possible" but needs bundler setup. New in 0.2.128: `--split-debug-info`, `--ts-typed-array-buffers`.
- `--remove-name-section --remove-producers-section` saved about 100 B on a toy crate.

### Rust wasm target, size, and panics

- Default target features from `rustc --print cfg` (1.99): bulk-memory, multivalue, mutable-globals, nontrapping-fptoint, reference-types, sign-ext. reference-types and multivalue became defaults in 1.82 (https://blog.rust-lang.org/2024/09/24/webassembly-targets-change-in-default-target-features/); bulk-memory and nontrapping-fptoint in 1.87 with LLVM 20 (https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html).
- `wasm32v1-none` is the `no_std` MVP + mutable-globals target; `-Ctarget-cpu=mvp` needs nightly build-std. Neither is needed for current browsers.
- **wasm-opt** must be told the features: plain `wasm-opt -Oz` (binaryen 129) fails with "memory.copy operations require bulk memory operations". Use `--all-features` or explicit `--enable-bulk-memory --enable-nontrapping-float-to-int --enable-reference-types --enable-multivalue --enable-sign-ext --enable-mutable-globals`.
- wasm-bindgen does no optimization itself; its guide says it only strips unneeded bindings.
- `strip = true` did not break wasm-bindgen's custom sections.
- **Sizes**, toy crate (`add`, `greet`), `opt-level = "s"`, LTO, `codegen-units = 1`, strip, `panic = "abort"`:

  | Stage | Size |
  | --- | --- |
  | cargo output | 30,130 B |
  | after wasm-bindgen | 15,562 B |
  | + `--remove-name-section --remove-producers-section` | 15,457 B |
  | then `wasm-opt -Oz --all-features` | 13,768 B (6,255 B gzipped) |

  `wasm-opt` alone on the 15,562 B file gave 13,830 B (about 11% smaller).
- **Sizes**, larger demo crate (struct, enum, imports, etc.), LTO, `codegen-units = 1`, strip:

  | opt-level | raw | after wasm-bindgen | after `wasm-opt -O` |
  | --- | --- | --- | --- |
  | "s" | 41,599 B | 20,981 B | 19,109 B |
  | "z" | 43,185 B | 22,378 B | 20,034 B |
  | 3 | 39,436 B | 19,393 B | 17,948 B |

  On this crate `"z"` was largest and `3` smallest. Start with `"s"` and re-measure with real code.
- **Panics:** `panic = "abort"` is the wasm32-unknown-unknown default (target spec `"panic-strategy": "abort"`), so setting it in the profile is redundant and leaving it out keeps native tests normal. A panic is a trap with no message; `console_error_panic_hook` 0.1.7 still works but is archived.
  - `panic = "unwind"` turns panics into JS `PanicError` exceptions and marks the instance terminated (wasm-bindgen 0.2.114+). It needs nightly + `-Zbuild-std=std,panic_unwind` (plus `-Cllvm-args=-wasm-use-legacy-eh=false`), and since 0.2.122 emits exnref exception handling, which needs Node 22.22.3+ / 24.15.0+. No plans to make it default. https://github.com/wasm-bindgen/wasm-bindgen/blob/main/guide/src/reference/catch-unwind.md
  - `--experimental-reset-state-function` still exists, plus `handler::schedule_reinit` and `set_on_abort` / `set_on_reinit` hooks.
  - The plan avoids panics instead: strict lints and `Result<_, JsError>`.

### Testing options

- Native `cargo test` for pure logic and doctests: passed.
- **Native tests can't cross into JS.** In the rust-overlay scratch flake (Rust 1.99.0, wasm-bindgen 0.2.129), native tests calling `add(2, 3)` and `parse_byte("7")` passed, but `parse_byte("300")`, whose error path calls `JsError::new`, panicked:
  ```
  thread 'tests::parse_error' panicked at .../wasm-bindgen-0.2.129/src/lib.rs:1227:1:
  cannot call wasm-bindgen imported functions on non-wasm targets
  ```
  The same applies to anything that creates or reads a `JsValue`, `JsError`, `js_sys`, or `web_sys` value. Hence the thin-wrapper pattern.
- **Host target is implicit.** The scratch `rust-toolchain.toml` listed only `wasm32-unknown-unknown` in `targets`, and native `cargo test` still compiled and ran: the host's std is always part of the toolchain.
- **wasm-bindgen-test 0.3.79** as a dev-dependency with `runner = "wasm-bindgen-test-runner"` under `[target.wasm32-unknown-unknown]` in `.cargo/config.toml`, then `cargo test --target wasm32-unknown-unknown`: ran under Node by default ("test greet_works ... ok").
  - `cargo test -q` fails because the runner rejects `--quiet`.
  - Plain `#[test]` items were not run on wasm ("no tests to run!").
  - `wasm_bindgen_test_configure!(run_in_browser)` or `WASM_BINDGEN_USE_BROWSER=1` with `CHROMEDRIVER` switches to a browser (not tested).
  - Doctests on wasm need Rust 1.89+.
  - https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/usage.html
- `wasm-bindgen --target nodejs` output called from Node worked (`5 Hello, node! 1 yes 3`).
- The plan skips wasm-bindgen-test: the Playwright test runs the real build in three browser engines, which covers the boundary better with no extra runner.

### Hygiene tools (not adopted yet)

- **cargo-deny** 0.20.2 (2026-07-09; nixpkgs 0.19.6): advisories, licenses, bans, sources. A tested `deny.toml` passed ("advisories ok, bans ok, licenses ok, sources ok") with `[graph] targets` for wasm32 + linux, `unmaintained = "all"`, `yanked = "deny"`, licenses `MIT`, `Apache-2.0`, `Unicode-3.0` (needed for unicode-ident), `multiple-versions = "warn"`, `wildcards = "deny"`, unknown registries/git denied. The current format has no `version = 2` key. https://embarkstudios.github.io/cargo-deny/checks/cfg.html
- **cargo-shear** 1.14.0 (2026-09-22; nixpkgs 1.13.4): unused deps on stable. Preferred over cargo-machete 0.9.2 (regex-based). Both reported clean.
- cargo-udeps 0.1.61 needs nightly. cargo-hack 0.6.45 is pointless without features. cargo-nextest 0.9.148 not needed.

### Vite 8

Changelog: https://raw.githubusercontent.com/vitejs/vite/main/packages/vite/CHANGELOG.md. Current Vite is 8.3.4 (2026-10-08).

- **8.0.0:** SSR support for `.wasm?init` (#21102).
- **8.1.0** (2026-06-23): "direct .wasm imports (WASM ESM Integration)", https://github.com/vitejs/vite/pull/21779 (merged 2026-06-11).
- **8.2.0:** unwraps `WebAssembly.Global` exports, enables js-string builtins (#22674).
- **8.3.3 / 8.2.4 / 8.1.6** (2026-10-06): fix GHSA-vfpm-58rq-9qcg, `?vite-wasm-instance` requests bypassed `server.fs` checks (https://github.com/vitejs/vite/pull/23655). The repo is on 8.2.2 with `host: true` and `allowedHosts: true`. astro 7.3.8 requires vite ^8.3.1.
- The feature is Vite's own `vite:wasm-helper` plugin, which parses the binary and generates glue with top-level await. Rolldown's old wasm-fallback plugin was removed (rolldown #9775).
- From `dist/node/chunks/node.js` (8.2.2): client uses `fetch` + `instantiateStreaming`; server (`consumer === 'server'`) uses `readFile(new URL(file, import.meta.url))`, or a base64 `data:` URL if inlined; dev serves a file URL.
- Vite docs: "a directly imported `.wasm` file behaves as an async module and requires top-level `await`"; server-side only in Node-compatible runtimes. https://vite.dev/guide/features#webassembly
- `?init` returns a function giving a `WebAssembly.Instance`; `?url` gives the asset URL.
- Plain `vite build --ssr` needs `build.ssrEmitAssets: true`, otherwise ENOENT on the `.wasm`.
- Client build output for the toy crate: `addc_bg-*.wasm` 15.56 kB (6.71 kB gzipped).
- **Source-phase imports are not supported** by Vite/Rolldown:
  ```
  [MISSING_EXPORT] "default" is not exported by "src/pkg-bundler/hello_bg.wasm".
     │ import source mod from "../pkg-bundler/hello_bg.wasm";
  ```
- **Browsers:** Chromium 154 headless supports neither feature natively, so Vite's glue is required:
  - `import source m from './hello_bg.wasm'` → `Uncaught SyntaxError: Unexpected identifier 'm'`
  - bundler glue imported directly → `TypeError: Failed to fetch dynamically imported module`
  - Firefox and Safari not tested.
- **Environment API** (https://vite.dev/guide/api-environment-plugins): `configEnvironment(name, config, env)` is the documented place for per-environment settings. `hotUpdate` runs once per environment with `this.environment`. `buildStart` and `watchChange` run only for the client environment in dev unless `perEnvironmentStartEndDuringDev` / `perEnvironmentWatchChangeDuringDev` are set.
- Running an external build synchronously in `resolveId` blocks every resolve, runs per environment, and fires repeatedly; avoid it.
- Rerunning `wasm-bindgen` into a watched directory during `astro dev` worked: Vite logged `watch crates/hello/pkg/hello_bg.wasm` and the next request served the new output, server-rendered and client-side.

### Node

- Wasm ESM integration unflagged since **v24.5.0 / v22.19.0**; instance-phase imports are Stability 1.1, source phase (`import source`, added v24.0.0) is 1.2; JS string builtins are on automatically. https://nodejs.org/api/esm.html#wasm-modules
- On Node 26.10.0, importing bundler `pkg/hello.js` directly gave `5 Hi, node!` plus the ExperimentalWarning; `import source` returned a `WebAssembly.Module`.
- Vite's server build and dev module runner inline their own glue, so they don't depend on Node's support (unless the package is externalized).

### Astro 7

- **Docs:** only "Imports reference → WASM" with `WebAssembly.instantiateStreaming(fetch('/example.wasm'))` (https://docs.astro.build/en/guides/imports/#wasm). The Cloudflare adapter documents `.wasm?module`. No wasm integration.
- **Prerender bug:** in `dist/core/build/static-build.js`, static output runs `ssrMoveAssets()` before `generatePages()`; server output runs them the other way. The prerender chunk still points at `new URL("../_astro/hello_bg.CN7ZGbTL.wasm", import.meta.url)` inside `dist/.prerender/`, which has been moved to `dist/_astro/`:
  ```
  ├─ /sub/nested.html [ERROR] Error: ENOENT: no such file or directory, open '.../site/dist/.prerender/_astro/hello_bg.CN7ZGbTL.wasm'
  [ERROR] [build] Caught error rendering /sub/nested: Error: ENOENT: ...
  ```
  - Reproduces with `build.format` `'file'` and `'directory'`, on astro 7.3.5 (vite 8.2.2) and 7.3.8 (vite 8.3.4). The code is unchanged in 7.3.8.
  - `--target web` with `readFileSync(new URL(..., import.meta.url))` in frontmatter fails the same way.
  - A web search found no existing issue (inconclusive).
- **`vite.environments.prerender.build.*` is silently dropped:** `core/build/vite-build-config.js` rebuilds `environments.prerender.build` and only keeps `rolldownOptions.output`. A debug plugin printed `DBG prerender 4096` after setting `assetsInlineLimit` there. A plugin's `configEnvironment` does take effect.
- **Fix (tested):** a plugin returning `{ build: { assetsInlineLimit: (f) => f.endsWith('.wasm') || undefined } }` from `configEnvironment` for non-client environments. Results: `/index.html` contained `Hello, ssr! 5`, Svelte island SSR output `Hello, svelte! 1`, `/sub/nested.html` contained `2`. In `astro preview` with headless Chromium: `<p id="client">Hello, client! 42</p>`, both `client:load` and `client:only` rendered, `/_astro/*.wasm` served as `application/wasm`.
  - Inlining costs nothing in output since `dist/.prerender` is deleted after generation. With a server adapter it would embed base64 in the server bundle.
  - A global `build.assetsInlineLimit` also works but inlines the wasm into client JS too.
  - `astro check` flags (TS 7030) a `configEnvironment` that doesn't return on every path; add an explicit `return undefined`.
- **Option B (tested):** externalize the package for non-client environments (`resolve: { external: ['hello'] }`), so Node loads it natively. Build passed (`Hey, pkg-ssr! 5 2`) with the ExperimentalWarning. Would probably defeat dev module invalidation (untested).
- **Didn't work:** top-level `vite.ssr.external: ['hello']` (ignored for prerender, same ENOENT); `--target web` with `await init()` in frontmatter (`TypeError: fetch failed … Caused by: not implemented... yet...`).
- `astro dev` server rendering worked without the fix; the client got `import * as wasm from "/src/wasm/addc_bg.wasm?import"`.
- `astro check` runs integrations' `astro:config:setup` with `command = "sync"`.
- `astro check` does not type-check `.svelte` files: a deliberate error in `Counter.svelte` was reported only by `svelte-check`. The repo's `test:astro` misses Svelte type errors today.
- Astro 7's `astro dev` backgrounds itself by default (`Stop: astro dev stop`); `ASTRO_DEV_BACKGROUND=1` keeps it in the foreground, like the existing `ASTRO_PREVIEW_BACKGROUND=1` in the Playwright config.

### Import styles compared

All tested for build, dev, and types.

| Style | Result |
| --- | --- |
| Relative path to `pkg/x.js` | Works; types from wasm-bindgen's `.d.ts`; `astro check` caught `Type 'number' is not assignable to type 'string'`. Long paths. |
| **package.json `"imports"`** (chosen) | Works in Vite build and dev; `astro check` and `svelte-check` resolve types; no plugin, no `.d.ts`, no `allowArbitraryExtensions`; resolved lazily so `pnpm install` doesn't need `pkg/`. Wildcard `"#rust/*": "./crates/*/pkg/index.js"` tested. |
| pnpm workspace (`packages: [crates/*/pkg]`, `"hello": "workspace:*"`) | Works, linked as source so the dep optimizer leaves it alone, but `pkg/` must exist before `pnpm install`. |
| Virtual `rust:x` module or `.rs` imports | Needs a plugin plus types via tsconfig `paths` or `allowArbitraryExtensions` + `lib.d.rs.ts`; ambient `declare module` can't re-export relative paths. More machinery for no gain. |

### Existing plugins (npm, 2026-10-12)

| Package | Version | Last modified | Notes |
| --- | --- | --- | --- |
| vite-plugin-wasm | 3.6.0 | 2026-03-15 | Peer vite ^2–^8; redundant on Vite 8.1+. Still the option for Astro 6 (Vite 7). |
| vite-plugin-top-level-await | 1.6.0 | 2025-07-17 | Redundant; Astro targets esnext. |
| @wasm-tool/rollup-plugin-rust | 3.1.6 | 2026-06-05 | Imports `Cargo.toml`. Built on Astro 7.3.8 / Vite 8.3.4 (prior design) but inlined the wasm as base64 in the JS bundle and gave no types. |
| vite-plugin-rsw | 2.0.11 | 2026-03-21 | |
| vite-plugin-wasm-pack | 0.1.12 | 2022 | Abandoned. |
| @rolldown/plugin-wasm | | | Doesn't exist (404). |

None addresses the Astro prerender bug.

### Prior design (from an earlier chat)

A handoff page from an earlier chat ("Import Rust files directly in Astro", tested 2026-10-10 on Astro 7.3.8, Vite 8.3.4 / Rolldown 1.2.13, wasm-pack 0.15.0, wasm-bindgen 0.2.129, rustc 1.97.0, @astrojs/svelte 9.0.1, Svelte 5.57.2, TypeScript 6.0.3, Node 22).

- **Design:** a 58-line Vite plugin, `rust.mjs`. On a `.rs` import, `resolveId` walks up to the nearest `Cargo.toml`, runs `wasm-pack build --target bundler` synchronously once per crate (`--dev` under `astro dev`), and returns `pkg/<name>.js`. It writes `lib.d.rs.ts` next to `lib.rs` (`export * from '../pkg/<main>'`) for types with `allowArbitraryExtensions`. `configEnvironment` inlines `.wasm` for non-client environments. `server.watcher` rebuilds on `.rs` / `Cargo.toml` changes; `server.watch.ignored` covers `**/target/**`.
- **Its test results** (headless Chromium, `astro dev` and `astro build` + `astro preview`):
  - Client script, production: separate hashed `.wasm` in `dist/_astro/`, served as `application/wasm`.
  - Dev edit to `lib.rs`: page reloaded in about 0.6–0.8 s.
  - Rust compile error in dev: printed in the terminal; page kept the last good build and recovered.
  - Frontmatter only: built to a single 141-byte `index.html`; frontmatter output also updated after a Rust edit in dev.
  - Svelte `client:load`: Rust output in built HTML, interactive after hydration, no hydration warnings. `client:only`: browser only.
  - Svelte dev edit to `lib.rs`: updated in place without a page reload; the component remounts, so state resets.
  - `astro check` flagged `add("2", 3)` in `.astro` files; `svelte-check` needed for `.svelte`.
- **Its caveats:**
  - No browser APIs in code that runs at build time (frontmatter, server render of `client:load` islands). Shared functions should take data and return plain values.
  - Always call imported functions directly; hot updates replace the module, so cached references go stale.
  - Crate must live inside the project so Vite's watcher sees it.
  - On a fresh checkout, types point into gitignored `pkg/`, so `astro build` had to run before `astro check`.
  - `execFileSync` pauses the dev server while cargo compiles.
  - Netlify, Vercel, and Cloudflare Pages build images weren't checked; the build host needs the Rust toolchain.
  - Pre-rendering an interactive: import the crate in frontmatter for initial markup and again in the client script; the two copies share no memory, so pass state through the HTML.
- **Its alternatives:** separate `wasm-pack` step + `init()` per crate (works, more boilerplate and a separate watch process); `vite-plugin-wasm-pack` and `vite-plugin-rsw` stale; `vite-plugin-wasm` not needed on Vite 8.1+.
- **What this plan keeps and changes:** keeps native Vite wasm imports, the `configEnvironment` inlining, and watcher-driven rebuilds. Replaces wasm-pack with `cargo` + `wasm-bindgen` from Nix, the `.rs` resolver and `lib.d.rs.ts` with the `#rust/*` alias, and the synchronous `resolveId` build with async builds in Astro hooks. Building on `astro:config:setup` also removes the "build before check" ordering problem.

### CI and deploy speed

#### Current state (GitHub API, runs on 2026-10-11)

- CI runs only on `pull_request`, on `ubuntu-24.04-arm`. Latest run, 67 s total: set up 1 s, checkout 4 s, install Nix 3 s, **install packages (`nix develop -c pnpm install`) 30 s**, cache images 2 s, **tests 24 s**. Recent runs took 72–75 s; two earlier ones 3.5–4.5 min.
- Nothing caches the Nix store: `cachix/install-nix-action` only installs Nix, and every run downloads the devShell from cache.nixos.org. The only `actions/cache` step caches Astro's optimized images.
- Deploys use Cloudflare Pages' git integration (check run app "Cloudflare Workers and Pages", project `sullvn`, previews at `<hash>.sullvn.pages.dev`), building in Cloudflare's image. The dashboard build command and image version are unknown.

#### Measured Rust cost (this machine: 4 cores, 16 GB, aarch64, same as the GitHub ARM runner)

Toolchain (Rust 1.99.0 via rust-overlay):

| Download (xz) | Size |
| --- | --- |
| rustc | 66 MB |
| rust-std (aarch64) | 31 MB |
| rust-std (wasm32) | 23 MB |
| cargo | about 11 MB |
| rust-analyzer | 8 MB |
| rust-src | 6 MB |
| clippy | about 4 MB |
| rustfmt | 2 MB |

- About 150 MB to download, about 1.2 GB unpacked (`nix path-info -S` of `rust-minimal-1.99.0`). wasm-bindgen-cli 0.2.129 is 82.5 MB and binaryen 132 is 90.4 MB unpacked, both from cache.nixos.org.
- rust-overlay builds the toolchain on the runner from these tarballs (they're not on cache.nixos.org). `nix build --rebuild` of every toolchain derivation, one at a time, including re-fetching the tarballs: **17 s**. Real runs parallelize, so this is an upper bound. Separately, the research pass measured the rustc download at 0.4 s and its decompression at 2.7 s.

Cargo, smoke-style crate with wasm-bindgen 0.2.129, `CI=1`, fresh `target/`, without `--target host-tuple`:

| Command | Cold | No changes | After editing `lib.rs` |
| --- | --- | --- | --- |
| `cargo clippy --workspace --all-targets` | 4.8 s | 0 s | 0 s |
| `cargo clippy --workspace --target wasm32-unknown-unknown` | 5.6 s | | |
| `cargo test --workspace` | 4.8 s | 0 s | 0.2 s |
| `cargo doc --workspace --no-deps` | under 0.1 s | | |
| `cargo build --target wasm32-unknown-unknown` (dev) | 0.9 s | | |
| `cargo build --target wasm32-unknown-unknown --release` | 5.0 s | 0 s | 0.1 s |
| `wasm-bindgen` + `wasm-opt` | under 0.1 s | | |

- Cold total about 21 s; `target/` 352 MB. With cached dependencies, only our own crates rebuild (about 1–2 s).
- Estimate: with no caching, CI goes from about 67 s to about 105–110 s; with rust-cache, about 75–90 s.

#### Sharing build artifacts between cargo commands

Measured by the research pass (Rust 1.99.0, wasm-bindgen 0.2.129), counting units rebuilt from a fresh `target/`:

- Plain `cargo test`, `check`, `clippy --all-targets` (no `--target`): the wasm32 clippy run rebuilt all 19 units, proc-macros included. `check` after `test` rebuilt syn, quote, and the wasm-bindgen macros, because host-only builds give them a different profile.
- `clippy --all-targets` after `check`, then `doc --no-deps`, rebuilt only the smoke crate: those always share.
- **`--target host-tuple`** (a literal Cargo has accepted since 1.91) on native clippy, test, and doc: proc-macros and build scripts compiled once and were reused by `clippy --target wasm32`, the wasm dev build, native clippy, test, and doc. Each extra configuration rebuilt only about 6–7 target-side units. Doctests still ran.
- The release wasm build always rebuilt everything (20 units); tuning `[profile.release.build-override]` didn't make it share.
- Disk: 223 MB with `host-tuple`, 351 MB without.
- Cargo 1.99 (2026-10-01): "Incremental compilation is now disabled by default when running in CI. CI is detected via the CI environment variable" (#17220, https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html). GitHub Actions, Cloudflare Pages, and Workers Builds all set `CI=true`. 1.99 also added a built-in `debug` profile, identical to `dev` for now.

#### GitHub Actions caching

- **Scoping** (https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching): a run restores caches from its own branch, its base branch, or the default branch. A cache saved by a `pull_request` run is scoped to `refs/pull/N/merge` and "can only be restored by re-runs of the pull request". So with PR-only CI, no cache is shared across PRs, including the existing `astro-images` cache. A `push` (or scheduled) run on `main` fixes it.
- **Limits:** entries unused for 7 days are evicted; 10 GB per repo by default, raisable on paid storage (e.g. 50 GB for $2.80/month); 200 uploads/min, 1,500 downloads/min.
- **Cache service v2:** the legacy service was sunset 2025-02-01. actions/cache v5 moved to Node 24 (runner 2.327.1+); v6.0.0 (2026-06-23) moved to ESM; v6.1.0 (2026-06-26), the repo's version, handles read-only cache access.
- **Swatinem/rust-cache** v2.9.2 (2026-08-06): caches `~/.cargo` (registry cache, git deps, bin) and dependency artifacts in `./target`, not workspace crates. Skips `registry/src`, drops incremental files, prunes artifacts unused for a week. Key: job id, rustc release/host/hash, env vars matching `CARGO`, `CC`, `CFLAGS`, `CXX`, `CMAKE`, `RUST`, hash of all `Cargo.lock`/`Cargo.toml`, plus `rust-toolchain*` and `.cargo/config.toml`; falls back to caches from older lockfiles. `cache-on-failure` defaults to false; `save-if` controls saving; `cmd-format: nix develop -c {0}` supports Nix; sets `CARGO_INCREMENTAL=0` itself.
- **sccache** (sccache-action v0.0.11, 2026-07-29) with the GitHub cache backend: per-object and subject to rate limits; not worth it for a small workspace.

#### Nix store caching

- **magic-nix-cache-action** v15 (2026-09-09), active. Determinate announced end-of-life in February 2025, then reversed it: daemon v0.1.5 (2025-06-11) added "Support for GitHub Actions Cache v2", action v11 (2025-06-13) removed the deprecation warnings. Free; stores only paths not on cache.nixos.org (here, the rust-overlay toolchain). Same scoping rules, so it also needs the `main` run.
- **nix-community/cache-nix-action** v7.0.2 (2026-01-30): saves and restores all of `/nix` as one entry, with `gc-max-store-size` and purge options. README warns it "may slow down your workflow"; here it would also archive Chromium and the Playwright browsers, likely slower than cache.nixos.org.
- **cachix:** free 5 GB "for open source projects" (https://www.cachix.org/pricing); repo is public; cachix-action v17; needs an auth token secret. Paid prices aren't published.
- **FlakeHub Cache:** flakehub-cache-action v3.23.1 (2026-10-06); paid, pricing page didn't load (unverified).
- None of these overlaps with anything in the current workflow. magic-nix-cache is the only one that would cache exactly the missing piece without re-caching what cache.nixos.org already serves.

#### Cloudflare Pages builds (checked 2026-10-12)

- **Build image** (https://developers.cloudflare.com/pages/configuration/build-image/, updated 2026-09-18): v3 is the last version ("There will be no further build image version changes"), with rolling updates. v1 projects were moved to v3 on 2026-09-15; v2 projects move on 2027-02-23. x86_64, in a gVisor container.
  - v3 preinstalls Go 1.24.3, Node 22.16.0, Bun 1.2.15, Python 3.13.3, Ruby 3.4.4, pnpm 10.11.1, Hugo 0.147.7, Zola 0.22.1, among others.
  - **No Rust, cargo, rustup, wasm32 target, or wasm-pack.** No Cloudflare build doc mentions Rust.
  - Build CPU and RAM aren't documented.
- **Limits** (https://developers.cloudflare.com/pages/platform/limits/): 20-minute build timeout; free plan 500 builds/month, 1 concurrent build.
- **Build cache** (https://developers.cloudflare.com/pages/configuration/build-caching/): only fixed directories (`.npm`, `.cache/yarn`, `.pnpm-store`, `.bun/install/cache`, and for Astro `node_modules/.astro`). No custom directories, so `~/.cargo`, `~/.rustup`, and `target/` can't be cached. 10 GB per project, purged 7 days after last read, needs build system v2+. Pointing `CARGO_HOME`/`RUSTUP_HOME`/`CARGO_TARGET_DIR` into `node_modules/.astro` might work (unverified).
- **Installing Rust there:** `curl https://sh.rustup.rs | sh -s -- -y --profile minimal`, then rustup reads `rust-toolchain.toml`. x86_64 downloads for 1.99.0: rustc 84 MB, std 31.6 MB, wasm32 std 23.2 MB, cargo 11 MB, plus clippy, rustfmt, rust-src, rust-analyzer: about 165 MB of xz per build. Estimated 30–60 s per build (unverified). rustup installs every listed component, and `channel = "stable"` resolves to the current release on the day, not what `flake.lock` pins, so the toolchain could drift from CI's.
- **Prebuilt tools:** wasm-bindgen 0.2.129 ships only a musl build for x86_64 Linux (`wasm-bindgen-0.2.129-x86_64-unknown-linux-musl.tar.gz`, 11.7 MB, with `.sha256sum`; https://github.com/wasm-bindgen/wasm-bindgen/releases/tag/0.2.129). binaryen version_133 is `binaryen-version_133-x86_64-linux.tar.gz`, 111 MB (https://github.com/WebAssembly/binaryen/releases/tag/version_133); lighter options are npm `binaryen@132.0.0` (JS/wasm, slower) or `binaryen-version_133-node.tar.gz` (2.3 MB).
- **Pages vs Workers:** the Pages docs home now opens with "Are you sure you want to use Pages? … [Workers] is Cloudflare's primary platform for building applications. Start new projects with Workers." (https://developers.cloudflare.com/pages/, updated 2026-08-25). Not formally deprecated; the build-image policy runs through 2027.
- **Workers Builds** (https://developers.cloudflare.com/workers/ci-cd/builds/build-image/, `/limits-and-pricing/`): Ubuntu 24.04 x86_64, Node 24.18.0, no Rust. Free 2 vCPU, paid 4 vCPU; 8 GB RAM, 20 GB disk, 20-minute timeout; 3,000 build minutes/month free, 6,000 paid. Same fixed-directory cache, so moving to Workers doesn't solve the Rust problem.

#### Deploying from GitHub Actions

- `cloudflare/wrangler-action` v4.1.3 (2026-09-24), defaults to Wrangler v4; npm wrangler is 4.149.0.
- A git-connected Pages project "cannot switch to Direct Upload later" (https://developers.cloudflare.com/pages/configuration/git-integration/), but turning off automatic deployments (Settings > Build > Branch control: untick automatic production deployments, preview branches "None") lets "you use Wrangler to deploy directly to your Pages project". The project, domain, and URLs stay. Drag-and-drop uploads don't work on git projects; wrangler does.
- `wrangler pages deploy dist --project-name=sullvn --branch=<name>` gives `<branch>.sullvn.pages.dev`; wrangler detects the branch from git. Also `--commit-hash`, `--commit-message`, `--commit-dirty`.
- Secrets (https://developers.cloudflare.com/pages/how-to/use-direct-upload-with-continuous-integration/): `CLOUDFLARE_API_TOKEN` (Account > Cloudflare Pages > Edit), `CLOUDFLARE_ACCOUNT_ID`; workflow permission `deployments: write` and `gitHubToken` for a GitHub Deployment. Action outputs `deployment-url` and `pages-deployment-alias-url`.
- Lost: the Cloudflare bot's PR comment and check run (a sticky-comment action could replace the comment). Fork and Dependabot PRs get no preview (no secrets).
- **Workers static-assets equivalent:** `wrangler.jsonc` with `"assets": {"directory": "./dist"}`, `wrangler deploy` for production, **Workers Previews** (`wrangler preview --name pr-N`; docs say Wrangler 4.135.0+, the action README says 4.136.3) at `<preview>-<worker>.<subdomain>.workers.dev`, 100 previews per Worker on the free plan, token template "Edit Cloudflare Workers". https://developers.cloudflare.com/workers/previews/, https://developers.cloudflare.com/workers/static-assets/migration-guides/migrate-from-pages/

### Tested in scratch projects (this chat)

- **rust-overlay** with `channel = "stable"`: Rust 1.99.0, release wasm build, `wasm-bindgen` 0.2.129, `wasm-opt` with the six feature flags; no separate `lld`.
- **Integration prototype** (Astro 7.3.5, Vite 8.2.2, wasm-bindgen 0.2.127 from the locked nixpkgs), with a crate exporting `add`, `greet`, `sum(&[f64])`, and `parse_byte -> Result<u8, JsError>`:
  - `#rust/*` wildcard imports resolve; `const x: string = add(1, 2)` fails `astro check` with TS2322.
  - `astro check` logged `config:setup command=sync` and `built crates (debug) in 8408ms` before type-checking.
  - `astro build` produced `<p id="ssr">Hello, ssr! 5 4 number too large to fit in target type` (the thrown `JsError` message from `parse_byte('300')`) and `dist/_astro/index_bg.CjxKB4-W.wasm`.
  - In `astro dev`, a Rust edit was live in about 2 s (`built crates (debug) in 135ms`). A syntax error logged cargo's `unclosed delimiter` error, the page kept serving, and it recovered on the next save.
- Svelte islands with `client:load` and `client:only` worked in build and preview (headless Chromium).

### Not yet verified

- That an already-open tab updates during dev (only fresh requests checked).
- That Firefox and WebKit run the client wasm (covered by the new Playwright test).
- That the flake change works in CI.
- Actual CI timings with Rust on the GitHub runner, and how much magic-nix-cache would save over rust-overlay's CDN downloads.
- Cloudflare's Rust install cost, Pages build CPU/RAM, and the `node_modules/.astro` cache hack (moot if deploys move to GitHub Actions).
- The current Pages dashboard build command and image version.
- That the `dist/` built for Playwright is identical to a production build (no difference in build env is visible).
- Server-adapter output (not needed for this static site).
- Whether an Astro issue for the prerender bug already exists.
