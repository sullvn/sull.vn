**GitHub Actions CI proposal for sullvn/sull.vn**

Reviewed 8 September 2026 · Revised 9 September 2026 after review · Proposal only · Temporary review document; remove before merging

I recommend one required GitHub Actions job, `test`, which installs dependencies and runs `pnpm test` inside the repository’s locked Nix development shell on an arm64 Ubuntu runner. Package scripts own the checks; an active branch ruleset requires their successful completion before merging into `main`. The review found three problems that would have failed the first run as originally proposed: the repository stores its images and screenshot baselines in Git LFS, a cold production build takes about two minutes and exceeds Playwright’s server timeout, and the committed baselines come from an arm64 machine. The revised design fetches LFS objects, builds before starting Playwright, matches the baseline architecture, and pins fonts through Nix instead of relying on host fonts.

The existing suite already covers unit tests, Astro diagnostics, a production build, and six Chromium/Firefox visual cases across four routes. The gap is that `pnpm test` omits Biome. Preserve that coverage and the committed Linux screenshots while making the aggregate command complete. [Package scripts](https://github.com/sullvn/sull.vn/blob/9836e696c3a0c072968296ecc4dbb17a663427ac/package.json), [Playwright configuration](https://github.com/sullvn/sull.vn/blob/9836e696c3a0c072968296ecc4dbb17a663427ac/playwright.config.ts), [visual tests](https://github.com/sullvn/sull.vn/blob/9836e696c3a0c072968296ecc4dbb17a663427ac/tests/visual.spec.ts)

Use this package-script contract:

```json
{
  "build": "pnpm test:types && astro build",
  "test": "pnpm test:lint && pnpm test:unit && pnpm build && pnpm test:visual",
  "test:lint": "biome check . && actionlint",
  "test:types": "astro check",
  "test:unit": "node --test",
  "test:visual": "playwright test"
}
```

This is the relevant portion of `scripts`; preserve the other development and formatting commands. Retire `check`, whose operations now have named entries under `test:`. A search of the repository, including the agent configuration, found only `package.json` mentioning it, so no caller needs the old name.

The build is its own link in the chain, and the Playwright web server only starts `pnpm preview`. Diagnostics run once through `test → build → test:types`, and the production build runs once. This preserves the existing guarantee that `pnpm build` performs diagnostics, gives build and type failures their own place in the job log, and keeps `pnpm test:types` available for fast, targeted feedback. `pnpm test:visual` on its own serves whatever `dist/` exists; run `pnpm build` or `pnpm test` for a fresh one. The inspected Astro checker includes the repository’s TypeScript files and Astro components, so another TypeScript checker would duplicate current coverage.

The build moved out of the server command because of a measurement taken in a fresh copy of the tracked files:

| Command, fresh checkout, no cache | Time |
| --- | --- |
| `astro check` | 6.5 s |
| `astro build` without an image cache | 2 min 2 s |
| `astro build` again with the image cache | 2.8 s |

The earlier 15-second figure ran with a warm image cache: `node_modules/.astro` in the development checkout holds 182 transformed images. CI never has that cache, and Playwright waits 60 seconds for the web server by default, so a build inside the server command would have timed out on every run. Moving the build also removes a hidden coupling where type checking only ran because the visual suite happened to call `build`. Caching the Astro asset directory between runs could shrink the build to seconds; defer that until the measured runtime matters. [Playwright web server](https://playwright.dev/docs/test-webserver)

The explicit `&&` chain stops on failure and returns nonzero. A successful aggregate means every command completed successfully. Keep that list explicit when adding checks; a wildcard selector could unintentionally include a future snapshot-update or interactive script. A custom runner, composite action, or reusable workflow is unnecessary for this single job. [pnpm scripts](https://pnpm.io/cli/run), [GitHub’s guidance on repository scripts](https://docs.github.com/en/actions/how-tos/write-workflows/choose-what-workflows-do/add-scripts)

`actionlint` is the one additional checker: this change introduces workflow YAML, and actionlint understands its syntax, expressions, and job dependencies. Add `pkgs.actionlint` to the existing development shell and run it alongside Biome in `test:lint`. The locked nixpkgs package wraps the binary with ShellCheck and Pyflakes on its path. The review ran it against the proposed workflow inside a Git repository; it passed with the ShellCheck integration active. actionlint locates workflows from the nearest `.git` entry, which the repository provides. Biome 2.5.11 ignores YAML, so the two tools do not overlap. [actionlint checks](https://github.com/rhysd/actionlint/blob/main/docs/checks.md), [locked actionlint package](https://github.com/NixOS/nixpkgs/blob/42f17a57f4f6e33b3de3dca0a2a5ea5233169d02/pkgs/by-name/ac/actionlint/package.nix)

The Nix shell remains the tool definition. The committed lockfile selects Node 26.8.1, pnpm 11.25.0, Biome 2.5.11, and Playwright’s browser package at 1.61.1; the npm manifest pins `@playwright/test` to 1.61.1. Preserve the existing browser path, download suppression, and Nix host-validation settings. Update the npm Playwright dependency and Nix browser package together when upgrading. cache.nixos.org holds every development-shell package at the locked revision for both `x86_64-linux` and `aarch64-linux`; a fresh runner downloads about 490 MiB and unpacks 3.6 to 3.9 GiB. The system Chromium package, used only by the MCP configuration, accounts for a 1.7 GiB unpacked closure, and a browsers override without WebKit and ffmpeg would save about 70 MiB of download. A separate CI shell could drop both; defer unless measured runtime warrants it. [Repository flake](https://github.com/sullvn/sull.vn/blob/9836e696c3a0c072968296ecc4dbb17a663427ac/flake.nix), [locked Playwright package](https://github.com/NixOS/nixpkgs/blob/42f17a57f4f6e33b3de3dca0a2a5ea5233169d02/pkgs/development/web/playwright/driver.nix)

Use `nix develop --no-update-lock-file` and `pnpm install --frozen-lockfile` to reject dependency definitions that would require lockfile changes. Install the normal development dependencies with the existing package-build allowlist. Nix supplies tools and browsers; pnpm supplies the application’s JavaScript dependencies. Keep `pkgs.chromium` in the default shell: the visual suite uses Playwright’s browser bundle, but the repository’s MCP configuration uses that separate executable. `nix develop --command` runs the shell hook and preserves the caller’s environment, including `CI`; the review confirmed both. [Nix develop reference](https://nix.dev/manual/nix/2.34/command-ref/new-cli/nix3-develop), [pnpm installation](https://pnpm.io/cli/install)

The repository tracks `*.png`, `*.jpg`, and `*.avif` with Git LFS. All 20 source images and all six baselines are pointer files in Git. Checkout does not download LFS objects by default, so the original job would have built against three-line pointer files: sharp cannot decode them, and Playwright cannot read the baselines. Set `lfs: true` on the checkout step. LFS downloads from GitHub Actions count against the repository owner’s bandwidth quota.

| Item | Value |
| --- | --- |
| LFS objects at the inspected commit | 26 files, about 88 MB |
| Free LFS bandwidth per month on the personal plan | 10 GiB |
| Cold runs before reaching the quota | about 115 |

With a zero-dollar budget, exceeding the quota blocks LFS for the rest of the month, including local pulls. With Dependabot off and a single maintainer, the expected run count stays well under that; watch usage on the repository owner’s billing page. If it trends toward the cap, cache `.git/lfs` keyed on the LFS object list, using checkout without LFS followed by `git lfs pull`, and confirm that anonymous LFS download works for this public repository with unpersisted credentials before relying on it. [LFS storage and bandwidth](https://docs.github.com/en/repositories/working-with-files/managing-large-files/about-storage-and-bandwidth-usage), [LFS attributes](https://github.com/sullvn/sull.vn/blob/9836e696c3a0c072968296ecc4dbb17a663427ac/.gitattributes), [checkout inputs](https://github.com/actions/checkout/blob/3d3c42e5aac5ba805825da76410c181273ba90b1/action.yml)

The site uses two system-font stacks. On the development host, fontconfig sees exactly one font file, DejaVu Sans from nixpkgs’ minimal DejaVu package, and every family in both stacks, including the monospace stack, resolves to it. The committed baselines therefore render entirely in DejaVu Sans under fontconfig’s stock rendering rules: slight hinting, antialiasing, no subpixel rendering. An Ubuntu runner ships DejaVu, Liberation, and Noto families under `/usr/share/fonts`, and the Nix-built browsers read the host’s fontconfig, so the runner would choose different families and produce different screenshots.

Pin the fonts through Nix instead. The locked font helper accepts `impureFontDirectories` and `includes`; with both empty, the review confirmed the generated file lists only the DejaVu minimal directory and the user’s XDG font directory, and no configuration includes. Point `includes` at the fontconfig package’s own `etc/fonts/conf.d` to keep the stock rendering rules, which are the same files NixOS installs. Export the result in the shell hook:

```nix
export FONTCONFIG_FILE="${pkgs.makeFontsConf {
  fontDirectories = [ pkgs.dejavu_fonts.minimal ];
  impureFontDirectories = [ ];
  includes = [ "${pkgs.fontconfig.out}/etc/fonts/conf.d" ];
}}"
```

Both browsers honor `FONTCONFIG_FILE`, so the same closed font set applies on NixOS and on the runner. This is the Nix-native form of Playwright’s advice to generate and compare baselines in the same environment, and it replaces the shared-container follow-up from the original proposal. The remaining XDG directory is empty on the development host and on a fresh runner. Regenerate the six baselines locally if the pinned configuration changes their rendering, and review that diff on its merits. [Playwright visual comparisons](https://playwright.dev/docs/test-snapshots), [locked font helper](https://github.com/NixOS/nixpkgs/blob/42f17a57f4f6e33b3de3dca0a2a5ea5233169d02/pkgs/development/libraries/fontconfig/make-fonts-conf.nix)

The development host is `aarch64-linux`, so the baselines come from arm64 builds of Chromium, Firefox, and the image pipeline. GitHub’s `ubuntu-24.04-arm` runner is free and unlimited for public repositories, with the same 4 vCPUs and 16 GB as the x64 image, and the Nix inputs are cached for it. Use it so the font pin controls the only remaining rendering variable. [Hosted runners](https://docs.github.com/en/actions/reference/runners/github-hosted-runners)

Keep visual tests required throughout. Use the first arm64 run to confirm the environment:

| Result of the first arm64 run | Action |
| --- | --- |
| All six cases pass | Keep the configuration and record the runtime. |
| A case differs | Read the report’s expected, actual, and diff images. A font difference means the pin is incomplete; anything else points at the browser build or image pipeline. Investigate before changing tolerances or baselines. |

Limit Playwright configuration changes to these settings:

| Setting | Value | Purpose |
| --- | --- | --- |
| `forbidOnly` | `!!process.env.CI` | Reject accidentally committed focused tests in CI. |
| `workers` | `process.env.CI ? 1 : undefined` | Follow Playwright’s CI recommendation for consistent resources across this small suite. |
| `use.trace` | `'retain-on-failure'` | Preserve a trace for diagnosing a failed visual case. |
| `reporter` | `github` in CI, alongside the existing HTML reporter | Annotate failures on the PR’s changed files while keeping the downloadable report. |
| `webServer.command` | `pnpm preview --port 4322 --ignore-lock` | Serve the output of the preceding `pnpm build` step. |

Playwright’s console fallback in CI is the terse dot reporter; the `github` reporter is the documented choice for Actions and combines with the HTML reporter. Piping server stdout is no longer needed because Astro’s diagnostics now print in the build step. Preserve the preview port, the `--ignore-lock` flag, the foreground-process environment setting, `reuseExistingServer: false`, and all six projects. [Playwright CI guidance](https://playwright.dev/docs/ci), [reporters](https://playwright.dev/docs/test-reporters), [recording options](https://playwright.dev/docs/test-use-options#recording-options)

Keep the default retry and snapshot settings. In installed Playwright 1.61.1, the HTML reporter’s exit hook returns before opening anything when `CI` is set, and retries default to zero. Under the default `updateSnapshots: 'missing'`, a missing baseline is written and the test fails through a soft error, so an `updateSnapshots: 'none'` override is not needed to enforce failure. GitHub supplies `CI=true`, which the normal Nix command preserves. [Pinned HTML reporter](https://github.com/microsoft/playwright/blob/v1.61.1/packages/playwright/src/reporters/html.ts), [pinned reporter selection](https://github.com/microsoft/playwright/blob/v1.61.1/packages/playwright/src/runner/reporters.ts), [configuration defaults](https://playwright.dev/docs/api/class-testconfig), [missing snapshots](https://playwright.dev/docs/test-snapshots), [GitHub variables](https://docs.github.com/en/actions/reference/workflows-and-actions/variables)

The workflow only provisions dependencies, invokes the aggregate command, and preserves failure diagnostics:

```yaml
name: Checks

on:
  pull_request:
  push:
    branches: [main]

permissions:
  contents: read

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}

jobs:
  test:
    runs-on: ubuntu-24.04-arm
    timeout-minutes: 30
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          lfs: true
          persist-credentials: false

      - uses: cachix/install-nix-action@13d8dd58da0234aa297dedd986986ccb8e7f3e24 # v31.11.1

      - name: Install dependencies
        run: nix develop --no-update-lock-file -c pnpm install --frozen-lockfile

      - name: Run checks
        run: nix develop --no-update-lock-file -c pnpm test

      - name: Upload failure diagnostics
        if: failure()
        uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1
        with:
          name: playwright-report
          path: playwright-report/
          retention-days: 7
          if-no-files-found: ignore
```

The three full-SHA pins resolve to commits for their tags and were the latest releases on 9 September 2026: [checkout v7.0.1](https://github.com/actions/checkout/releases/tag/v7.0.1), [install-nix-action v31.11.1](https://github.com/cachix/install-nix-action/releases/tag/v31.11.1), and [upload-artifact v7.0.1](https://github.com/actions/upload-artifact/releases/tag/v7.0.1). With Dependabot off, bump them by hand; keep the adjacent version comments so a manual update is a two-token change.

`cachix/install-nix-action` installs upstream Nix and enables flakes and `nix-command`. The pinned action passes the workflow’s `GITHUB_TOKEN` to its installer, which uses it for GitHub access tokens when the `github_access_token` input is empty, so the explicit input is redundant. It needs no Cachix account. Checkout and artifact upload are GitHub-owned; the Nix bootstrap is the only third-party action. [Pinned action definition](https://github.com/cachix/install-nix-action/blob/13d8dd58da0234aa297dedd986986ccb8e7f3e24/action.yml), [pinned installer](https://github.com/cachix/install-nix-action/blob/13d8dd58da0234aa297dedd986986ccb8e7f3e24/install-nix.sh)

Upload the HTML report directory on failure. Playwright copies its screenshot and trace attachments into that report, so uploading the raw results directory as well is unnecessary for normal test failures. Ignoring a missing report lets an earlier lint, build, or installation failure stand on its own; those failures have their job logs. [HTML report attachment handling](https://github.com/microsoft/playwright/blob/v1.61.1/packages/playwright/src/reporters/html.ts), [workflow artifacts](https://docs.github.com/en/actions/concepts/workflows-and-actions/workflow-artifacts)

Use `ubuntu-24.04-arm` as an explicit OS generation and architecture; its hosted image still receives updates. Standard hosted execution for this public repository is free. Start with a 30-minute job timeout for cold provisioning and adjust after measurement. Nix’s normal binary substitutes and pnpm installation are sufficient initially; a cold run downloads about 490 MiB from cache.nixos.org plus the LFS objects. Add Nix or Astro caching only if measured runtime warrants it. One Linux job already runs both configured browsers. [Hosted runners](https://docs.github.com/en/actions/reference/runners/github-hosted-runners), [runner image updates](https://docs.github.com/en/actions/concepts/runners/github-hosted-runners), [cache behavior](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching)

Use the default `pull_request` activity types: opened, synchronize, and reopened, including draft PRs. The original proposal added `edited` to cover a PR retargeted at a different base. That also reruns on every title or description change and, with cancellation enabled, can cancel a run in flight. Retargeting is rare for this repository, the strict up-to-date rule forces a fresh run whenever the head lags its base, and the `shai` and `cowbox` workflows use the default set too. Preserve checkout’s default test-merge commit so checks cover the proposed change together with its current base. [Workflow events](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows)

Pushes to `main` validate its latest state after merging. Per-workflow, per-ref concurrency cancels superseded PR runs; the expression limits cancellation to pull requests, so a rapid sequence of merges to `main` still validates every intermediate state. [Concurrency](https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/control-workflow-concurrency)

This setup needs only read permission and no repository secrets. Fork PRs can use it, although outside contributors may require maintainer approval under the repository’s Actions settings. Full action pins and disabled credential persistence keep execution dependencies explicit. Dependabot is being turned off for this repository, so it no longer factors into workflow runs, LFS bandwidth, or the up-to-date rule; npm and action version bumps become manual, and any bump of `@playwright/test` must move together with the Nix browser package or the visual suite fails. The three Dependabot branches on the remote remain until closed. [Fork approvals](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/approve-runs-from-forks), [secure use](https://docs.github.com/en/actions/reference/security/secure-use)

Configure merge enforcement with an active branch ruleset:

| Rule or setting | Proposed value |
| --- | --- |
| Name | `Require checks` |
| Target | Default branch, currently `main` |
| Enforcement | Active |
| Require a pull request | Enabled |
| Required approvals | Zero |
| Required status check | Actual emitted job check `test` |
| Expected check source | GitHub Actions |
| Require branches to be up to date | Enabled |
| Bypass list | Empty, including administrators |
| Restrict deletions | Enabled |
| Block force pushes | Enabled |
| Restrict updates | Off |

Rulesets are available for this public personal repository. Requiring a PR and requiring a check are separate settings; zero approvals permits solo maintenance. Strict freshness requires incorporating new changes from `main` and rerunning checks before merging; each rerun costs a cold Nix download and an LFS fetch, which is acceptable at this repository’s PR rate now that Dependabot is off. An empty bypass list applies the rule to normal owner merges too; an owner with settings access can still edit the policy. “Restrict updates” would limit updates to bypass actors and should remain off. [About rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets), [available rules](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets)

GitHub requires named checks; it does not automatically require every independent check added later. Keep `test` as the stable check representing `pnpm test`, and select the emitted name observed in the actual run, rather than the workflow display name `Checks`. Selecting GitHub Actions as the source identifies the reporting integration; the workflow and scripts remain normal reviewable repository code. [Creating rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository), [check naming](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/troubleshooting-rules)

Keep the validation job unconditional, with no path filters or `continue-on-error`. GitHub accepts skipped or neutral required checks as well as successful ones, so skipping a job can satisfy a requirement. Conversely, filtering out an entire required workflow can leave its check pending. The single unconditional job and explicit script chain avoid these pitfalls. [Required-check behavior](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks)

A merge queue is unnecessary for this repository. The review could not confirm from the documentation whether personal accounts can enable one, and the answer does not change the recommendation. Add `merge_group` only if the repository later adopts a queue. [Merge queue setup](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-a-merge-queue)

Keep the implementation limited to these repository files and GitHub settings:

| Location | Work |
| --- | --- |
| `package.json` | Establish the aggregate and named checks; build before the visual suite. |
| `flake.nix` | Add `pkgs.actionlint`; export the pinned `FONTCONFIG_FILE`. |
| `playwright.config.ts` | Add focused-test protection, one CI worker, failure traces, the `github` reporter in CI, and a preview-only web server. |
| `.github/workflows/checks.yaml` | Add the workflow above. |
| `tests/visual.spec.ts-snapshots/` | Regenerate only if the font pin changes rendering. |
| GitHub repository settings | Activate and verify the required-check ruleset; turn Dependabot off. |

The current lockfiles need no updates merely to use packages from the already pinned nixpkgs input. Existing non-failing Biome schema and unsupported-Markdown notices do not require expanding this change into a lint-policy cleanup. The pnpm build allowlist names `protobufjs` and `re2`, neither of which appears in the lockfile; removing those entries is an optional, unrelated cleanup. [Build allowlist](https://github.com/sullvn/sull.vn/blob/9836e696c3a0c072968296ecc4dbb17a663427ac/pnpm-workspace.yaml)

Roll out and verify the implementation in this order:

1. Implement the repository changes and run the complete command locally in Nix. Run it once more after deleting `node_modules/.astro` and `dist/` to observe the cold build. Compare the baselines under the pinned font configuration and regenerate them if they change. Temporarily introduce a type/build error, verify that it fails the aggregate in the build step and prints useful diagnostics, then restore it. The standard lint and unit commands need no separate artificial-failure exercises.
2. Open the implementation PR and obtain a successful run on a fresh arm64 runner. Check that LFS objects were fetched, frozen installation, both browsers, all six visual cases, unchanged baselines, and cold runtime. Apply the first-run decision above if screenshots differ. A PR can run the workflow it introduces; a preliminary unprotected merge is unnecessary.
3. Select the emitted `test` check and GitHub Actions source, then activate the ruleset. GitHub requires a check to have completed successfully in the repository within the preceding seven days to be selectable.
4. On that same PR, push a deliberate visual failure. Confirm it blocks merging for the owner and produces a usable downloadable report with comparisons and a trace. Fix it and verify that the new run restores mergeability. This also checks that the previous successful result cannot satisfy a newer failing revision. Confirm strict freshness in the active ruleset; when `main` advances, incorporate it and rerun checks.
5. Remove the temporary proposal, obtain a final green run, and leave the implementation PR ready for review and merge with enforcement active. After the normal merge, confirm that the latest state of `main` has a green run, record the measured runtime, and note the LFS bandwidth the runs consumed.

Using the same PR avoids depending on a workflow that has not yet reached `main`. Committing YAML and enabling enforcement are distinct operations; a ruleset JSON file in the repository would not apply GitHub settings by itself. [PR workflow execution](https://docs.github.com/en/actions/reference/security/securely-using-pull_request_target), [required-check registration](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks), [ruleset configuration](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository)

The design follows your `shai` and `cowbox` precedents: ordinary repository commands, default PR and default-branch triggers, and releases kept separate. Nix supplies this repository’s tools as requested. [shai test workflow](https://github.com/sullvn/shai/blob/bb3382b83a3e7a7a163e92f286fe095dbe6c0e05/.github/workflows/test.yaml), [cowbox test workflow](https://github.com/sullvn/cowbox/blob/0deff1b6fb796e03a7c0b181dc08cdddb76bdebe/.github/workflows/test.yaml)

Research covered all 200 articles in the current GitHub.com Actions index, plus ruleset and tool documentation. Subsequent reviews checked the installed tools, script coverage, PR lifecycle, and enforcement sequence. A second review on 9 September 2026 measured a cold build, inspected the installed Playwright 1.61.1 sources, checked cache.nixos.org for both architectures, generated the pinned fontconfig file, linted the proposed workflow with the locked actionlint, and verified the action pins, the install-nix-action token handling, and the LFS configuration. Enterprise Server variants and external linked pages were outside the full-index review. [GitHub Actions documentation](https://docs.github.com/en/actions)

At inspected commit `9836e696c3a0c072968296ecc4dbb17a663427ac`, there were no tracked workflows. Public API responses identified a personally owned public repository with `main` unprotected and no rulesets. These observations establish why both code and GitHub settings are needed; no settings have been applied. [Repository metadata](https://api.github.com/repos/sullvn/sull.vn), [rulesets](https://api.github.com/repos/sullvn/sull.vn/rulesets), [main branch](https://api.github.com/repos/sullvn/sull.vn/branches/main)

Validation already performed: unit tests passed; `pnpm check` passed with zero Astro errors/warnings, four Biome unsupported-Markdown warnings, and one schema-version notice. All six visual cases passed in 15.1 seconds, including build and preview, in the existing development environment with a warm image cache. A fresh copy of the tracked files produced the cold build times above. Locked tool versions were evaluated separately. A fresh offline shell attempt reported missing derivations, so it did not establish a clean Nix bootstrap; the binary cache check covers that gap for both architectures. Fresh runner execution and actual merge enforcement still require implementation-time validation.

This revision changes only the proposal document. Its script, Nix, and workflow examples were checked structurally, and both the original and revised workflows passed the locked actionlint. No CI implementation or GitHub settings have been changed. The design assumes `main` is the branch to protect, zero human approvals suit solo maintenance, required checks apply to owner merges as well, and Dependabot is turned off.
