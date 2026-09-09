**GitHub Actions CI proposal for sullvn/sull.vn**

Reviewed 8 September 2026 · Revised 9 September 2026 after review · Proposal only · Temporary review document; remove before merging

Use one required GitHub Actions job, `test`, to install dependencies and run `pnpm test` in the repository’s locked Nix development shell on an arm64 Ubuntu runner. Package scripts define the checks; a branch ruleset requires them before merging into `main`. Fetch the Git LFS images, let Playwright build before serving the site, and control fonts and dates so local and CI screenshots agree.

The existing suite already covers unit tests, Astro diagnostics, a production build, and six Chromium/Firefox visual cases across four routes. The gap is that `pnpm test` omits Biome. Preserve that coverage and the committed Linux screenshots while making the aggregate command complete. [Package scripts](https://github.com/sullvn/sull.vn/blob/9836e696c3a0c072968296ecc4dbb17a663427ac/package.json), [Playwright configuration](https://github.com/sullvn/sull.vn/blob/9836e696c3a0c072968296ecc4dbb17a663427ac/playwright.config.ts), [visual tests](https://github.com/sullvn/sull.vn/blob/9836e696c3a0c072968296ecc4dbb17a663427ac/tests/visual.spec.ts)

Use this package-script contract:

```json
{
  "build": "pnpm test:types && astro build",
  "test": "pnpm test:lint && pnpm test:unit && pnpm test:visual",
  "test:lint": "biome check . && actionlint",
  "test:types": "astro check",
  "test:unit": "node --test",
  "test:visual": "playwright test"
}
```

This is the relevant portion of `scripts`; preserve the other development and formatting commands. Retire `check`, whose operations now have named entries under `test:`. A search of the repository, including the agent configuration, found only `package.json` mentioning it, so no caller needs the old name.

Keep the existing `pnpm build && pnpm preview --port 4322 --ignore-lock` web-server command. The full suite builds and checks types once, and starting Playwright through `test:visual`, a direct command, or an editor builds fresh output before serving it. This avoids a manual-build prerequisite for direct invocations and snapshot updates. A long-lived editor or UI session does not rebuild on every test rerun; restart its test server after changing site source. Astro already checks the repository’s TypeScript and Astro files, so another type checker would duplicate coverage.

Measurements taken in a fresh copy of the tracked files establish the cold-start budget:

| Command, fresh checkout, no cache | Time |
| --- | --- |
| `astro check` | 6.5 s |
| `astro build` without an image cache | 2 min 2 s |
| `astro build` again with the image cache | 2.8 s |

The earlier 15-second test run used cached images in `node_modules/.astro`. A fresh build exceeded Playwright’s default 60-second server timeout. Set `webServer.timeout` to `5 * 60 * 1000` initially and `webServer.stdout` to `'pipe'` so Astro’s build output and diagnostics appear in the job log. Five minutes is a starting value to validate against a cold runner, not a measured requirement. These two built-in settings address the timeout and diagnostics while keeping build orchestration inside Playwright. Add an image cache only if measured CI runtime warrants it. [Playwright web server](https://playwright.dev/docs/test-webserver)

The explicit `&&` chain stops on failure and returns nonzero. A successful aggregate means every command completed successfully. Keep that list explicit when adding checks; a wildcard selector could unintentionally include a future snapshot-update or interactive script. A custom runner, composite action, or reusable workflow is unnecessary for this single job. [pnpm scripts](https://pnpm.io/cli/run), [GitHub’s guidance on repository scripts](https://docs.github.com/en/actions/how-tos/write-workflows/choose-what-workflows-do/add-scripts)

`actionlint` is the one additional checker: this change introduces workflow YAML, and actionlint understands its syntax, expressions, and job dependencies. Add `pkgs.actionlint` to the existing development shell and run it alongside Biome in `test:lint`. The locked nixpkgs package wraps the binary with ShellCheck and Pyflakes on its path. The review ran it against the proposed workflow inside a Git repository; it passed with the ShellCheck integration active. actionlint locates workflows from the nearest `.git` entry, which the repository provides. Biome 2.5.11 ignores YAML, so the two tools do not overlap. [actionlint checks](https://github.com/rhysd/actionlint/blob/main/docs/checks.md), [locked actionlint package](https://github.com/NixOS/nixpkgs/blob/42f17a57f4f6e33b3de3dca0a2a5ea5233169d02/pkgs/by-name/ac/actionlint/package.nix)

The Nix shell remains the tool definition. The lockfile selects Node 26.8.1, pnpm 11.25.0, Biome 2.5.11, and Playwright browsers 1.61.1; npm pins `@playwright/test` to 1.61.1. Preserve the existing browser environment settings. Keep the npm and Nix Playwright versions together: either an npm update or `nix flake update` can break the pairing. The inspected Nix packages were available from cache.nixos.org for both Linux architectures, with about 490 MiB to download. Keep one shared shell and defer package trimming until runtime warrants it. [Repository flake](https://github.com/sullvn/sull.vn/blob/9836e696c3a0c072968296ecc4dbb17a663427ac/flake.nix), [locked Playwright package](https://github.com/NixOS/nixpkgs/blob/42f17a57f4f6e33b3de3dca0a2a5ea5233169d02/pkgs/development/web/playwright/driver.nix)

Use `nix develop --no-update-lock-file` and `pnpm install --frozen-lockfile` to reject dependency definitions that would require lockfile changes. Install the normal development dependencies with the existing package-build allowlist. Nix supplies tools and browsers; pnpm supplies the application’s JavaScript dependencies. Keep `pkgs.chromium` in the default shell: the visual suite uses Playwright’s browser bundle, but the repository’s MCP configuration uses that separate executable. `nix develop --command` runs the shell hook and preserves the caller’s environment, including `CI`; the review confirmed both. [Nix develop reference](https://nix.dev/manual/nix/2.34/command-ref/new-cli/nix3-develop), [pnpm installation](https://pnpm.io/cli/install)

The repository tracks `*.png`, `*.jpg`, and `*.avif` with Git LFS. All 20 source images and all six baselines are pointer files in Git. Checkout does not download LFS objects by default, so the original job would have built against three-line pointer files: sharp cannot decode them, and Playwright cannot read the baselines. Set `lfs: true` on the checkout step. LFS downloads from GitHub Actions count against the repository owner’s bandwidth quota.

| Item | Value |
| --- | --- |
| LFS objects at the inspected commit | 26 files, 87,507,548 bytes (83.45 MiB) |
| Free monthly LFS bandwidth for the owner’s personal account | 10 GiB, shared across repositories |
| Complete downloads if the entire allowance is unused | about 122 |

Check the owner’s current usage and actual budget during rollout. A PR run and its later `main` run already use two downloads; retries, local clones, forks, and other repositories share the allowance. Changed screenshots also add storage because older LFS versions remain. With a zero-dollar budget, exceeding the allowance blocks LFS until the next month. Add caching only if measured usage calls for it. [LFS billing](https://docs.github.com/en/billing/concepts/product-billing/git-lfs), [LFS attributes](https://github.com/sullvn/sull.vn/blob/9836e696c3a0c072968296ecc4dbb17a663427ac/.gitattributes), [checkout inputs](https://github.com/actions/checkout/blob/3d3c42e5aac5ba805825da76410c181273ba90b1/action.yml)

Cover local checkout as well as CI: add `pkgs.git-lfs` to the shared shell. Git alone can clone the repository with only pointer files, and installing JavaScript dependencies does not retrieve the images. Document the following sequence after installing Nix and Git and cloning the repository; the commands after `nix develop` run inside the shell:

```sh
nix --extra-experimental-features 'nix-command flakes' develop --no-update-lock-file
git lfs install --local
git lfs pull
pnpm install --frozen-lockfile
pnpm test
```

The first command enables the Nix features needed for this invocation; a local installation does not necessarily enable them by default. The CI installer already does. LFS initialization configures the local repository and its upload hook, while `git lfs pull` retrieves the existing source images and baselines. Verify the README works from a fresh clone without relying on the development host’s existing LFS or Nix configuration. [Installing Git LFS](https://docs.github.com/en/repositories/working-with-files/managing-large-files/installing-git-large-file-storage), [LFS collaboration](https://docs.github.com/en/repositories/working-with-files/managing-large-files/collaboration-with-git-large-file-storage), [Nix experimental features](https://nix.dev/manual/nix/2.34/development/experimental-features)

The development host resolves both system-font stacks to DejaVu Sans, including code text. Ubuntu has more fonts and may choose different ones. Math uses KaTeX’s separate web fonts, so the screenshots are not entirely DejaVu Sans. [KaTeX fonts](https://katex.org/docs/font.html)

Generate one small fontconfig file with `pkgs.writeText` and set `FONTCONFIG_FILE` in the shell. Include only `pkgs.dejavu_fonts.minimal` and the intended rendering rules: slight hinting, antialiasing, and no subpixel rendering. Exclude host and user font directories and settings. The earlier `makeFontsConf` example was incomplete: it kept the user font directory, and including the package’s whole `conf.d` directory loaded user settings through `50-user.conf`. A temporary user setting changed antialiasing from true to false under that configuration. Verify the replacement ignores both extra user fonts and user rendering preferences. A general font-configuration abstraction is unnecessary. [Fontconfig configuration](https://fontconfig.pages.freedesktop.org/fontconfig/fontconfig-user.html), [Nix font helper](https://github.com/NixOS/nixpkgs/blob/42f17a57f4f6e33b3de3dca0a2a5ea5233169d02/pkgs/development/libraries/fontconfig/make-fonts-conf.nix)

The pinned `dejavu_fonts.minimal` package contains only `DejaVuSans.ttf`, with no monospace, bold, or italic faces. Preserving this environment means preserving proportional code text and synthetic bold and italic styling for system-font text. These screenshots therefore provide limited coverage of intended monospace typography and distinctions between font weights. Keep that tradeoff explicit for this rollout; introducing the full family belongs in a separate visual change with reviewed baselines because it can alter line wrapping and page heights. [Pinned DejaVu package](https://github.com/NixOS/nixpkgs/blob/42f17a57f4f6e33b3de3dca0a2a5ea5233169d02/pkgs/data/fonts/dejavu-fonts/default.nix)

Make `formatDate()` in `src/utils/date.ts` format dates in UTC, matching `toIsoDay()`. It currently displays `2017-11-02` as November 2 in UTC but November 1 in Los Angeles. Those dates are written into the HTML during the build, so changing Playwright’s browser timezone would not fix them. Add a focused test that confirms the same date under both timezones. [Node timezone setting](https://nodejs.org/api/cli.html#tz)

Use `ubuntu-24.04-arm` to match the development host and committed baselines. This public repository gets 4 vCPUs and 16 GB without runner charges. Matching architecture and fonts reduces differences but does not prove identical rendering; operating-system settings and hardware can still matter. Confirm all six cases on the first fresh runner before accepting the environment. Regenerate baselines only for understood, reviewed changes. [Hosted runners](https://docs.github.com/en/actions/reference/runners/github-hosted-runners), [Playwright visual comparisons](https://playwright.dev/docs/test-snapshots)

Keep visual tests required throughout. Use the first arm64 run to confirm the environment:

| Result of the first arm64 run | Action |
| --- | --- |
| All six cases pass | Keep the configuration and record the runtime. |
| A case differs | Compare the expected, actual, and diff images. Check built content, system and KaTeX fonts, browsers, and images before changing tolerances or baselines. |
| An understood host-rendering difference remains after environment fixes | Use reviewed actual images from the CI report as candidate baselines, then require a normal passing CI run. Record the cause and whether local baseline generation still reproduces CI. |
| A difference remains unexplained | Keep the failure blocking and continue diagnosis; do not accept it by changing the baseline or tolerance. |

Headless rendering on `ubuntu-24.04-arm` with the committed Nix lockfile, Playwright version, and font configuration is authoritative for the required check. Local baseline generation is supported only when it reproduces that environment’s results. If an understood host difference prevents this, obtain candidate images from the existing failure report for the exact reviewed commit and map them to their corresponding baseline files; no automatic baseline-update workflow is needed. Architecture and font matching are prerequisites to validate, not a guarantee of identical rendering.

The existing snapshot filenames end in `linux`. Playwright 1.61.1 derives that suffix from `process.platform`, so Linux x64 and arm64 use the same baseline paths. Restrict generation to the supported environment in the README; the filenames do not enforce it. Add a small platform guard only if accidental updates become a practical problem, rather than introducing additional platform-specific baseline sets. [Pinned snapshot suffix](https://github.com/microsoft/playwright/blob/v1.61.1/packages/playwright/src/index.ts), [snapshot naming](https://playwright.dev/docs/test-snapshots)

Make baseline review concrete. GitHub may show only pointer changes for LFS images, and a passing run after replacing expected images does not show how they changed. Use this procedure:

1. Run `pnpm test:visual` against the existing baselines. Preserve the complete failure report, including its attachments, outside `playwright-report/` before another run replaces it; download CI artifacts before their seven-day retention expires when needed for review.
2. Open the saved report with `pnpm exec playwright show-report /path/to/saved-report`. Review expected, actual, and diff images and explain the intended changes.
3. In the validated local environment, run `pnpm test:visual --update-snapshots` and inspect the resulting baseline changes. If local rendering cannot reproduce CI, use the reviewed CI actual images instead. Accept only the corresponding intended changes.
4. Run `pnpm test` normally and keep the earlier comparison available for PR review. Commit and push changed baselines with LFS configured.

This procedure uses the existing report and artifact mechanism. No report-upload service or additional workflow is needed. [Reviewing Git LFS changes](https://docs.github.com/en/repositories/working-with-files/managing-large-files/collaboration-with-git-large-file-storage#viewing-large-files-in-pull-requests), [snapshot updates](https://playwright.dev/docs/test-snapshots#updating-screenshots)

Limit Playwright configuration changes to these settings:

| Setting | Value | Purpose |
| --- | --- | --- |
| `forbidOnly` | `!!process.env.CI` | Reject accidentally committed focused tests in CI. |
| `workers` | `process.env.CI ? 1 : undefined` | Follow Playwright’s CI recommendation for consistent resources across this small suite. |
| `use.trace` | `'retain-on-failure'` | Preserve a trace for diagnosing a failed visual case. |
| `reporter` | `github` in CI, alongside the existing HTML reporter | Annotate failures on the PR’s changed files while keeping the downloadable report. |
| `webServer.timeout` | `5 * 60 * 1000` | Allow the cold build and preview startup; validate the budget on the first fresh runner. |
| `webServer.stdout` | `'pipe'` | Show build progress and Astro diagnostics in the test output. |

Playwright’s console fallback in CI is the terse dot reporter; the `github` reporter is the documented choice for Actions and combines with the HTML reporter. Preserve the existing build-and-preview command, the preview port, the `--ignore-lock` flag, the foreground-process environment setting, `reuseExistingServer: false`, and all six projects. [Playwright CI guidance](https://playwright.dev/docs/ci), [reporters](https://playwright.dev/docs/test-reporters), [recording options](https://playwright.dev/docs/test-use-options#recording-options)

Keep the default retry and snapshot settings. In installed Playwright 1.61.1, the HTML reporter’s exit hook returns before opening anything when `CI` is set, and retries default to zero. Under the default `updateSnapshots: 'missing'`, a missing baseline is written and the test fails through a soft error, so an `updateSnapshots: 'none'` override is not needed to enforce failure. GitHub supplies `CI=true`, which the normal Nix command preserves. [Pinned HTML reporter](https://github.com/microsoft/playwright/blob/v1.61.1/packages/playwright/src/reporters/html.ts), [pinned reporter selection](https://github.com/microsoft/playwright/blob/v1.61.1/packages/playwright/src/runner/reporters.ts), [configuration defaults](https://playwright.dev/docs/api/class-testconfig), [missing snapshots](https://playwright.dev/docs/test-snapshots), [GitHub variables](https://docs.github.com/en/actions/reference/workflows-and-actions/variables)

The installed screenshot implementation already waits for `document.fonts.ready`, including for KaTeX’s web fonts. Keep the existing image-decoding preparation and avoid adding a duplicate font-wait helper. [Pinned screenshot implementation](https://github.com/microsoft/playwright/blob/v1.61.1/packages/playwright-core/src/server/screenshotter.ts)

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
  group: ${{ github.workflow }}-${{ github.event.pull_request.number || github.run_id }}
  cancel-in-progress: true

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

The three action pins match their release tags and were the latest releases on 9 September 2026: [checkout v7.0.1](https://github.com/actions/checkout/releases/tag/v7.0.1), [install-nix-action v31.11.1](https://github.com/cachix/install-nix-action/releases/tag/v31.11.1), and [upload-artifact v7.0.1](https://github.com/actions/upload-artifact/releases/tag/v7.0.1). Update the commit pins and their version comments together when upgrading manually.

`cachix/install-nix-action` installs upstream Nix and enables flakes and `nix-command`. The pinned action passes the workflow’s `GITHUB_TOKEN` to its installer, which uses it for GitHub access tokens when the `github_access_token` input is empty, so the explicit input is redundant. It needs no Cachix account. Checkout and artifact upload are GitHub-owned; the Nix bootstrap is the only third-party action. [Pinned action definition](https://github.com/cachix/install-nix-action/blob/13d8dd58da0234aa297dedd986986ccb8e7f3e24/action.yml), [pinned installer](https://github.com/cachix/install-nix-action/blob/13d8dd58da0234aa297dedd986986ccb8e7f3e24/install-nix.sh)

Upload the HTML report directory on failure. Playwright copies its screenshot and trace attachments into that report, so uploading the raw results directory as well is unnecessary for normal test failures. Ignoring a missing report lets an earlier lint, build, or installation failure stand on its own; those failures have their job logs. [HTML report attachment handling](https://github.com/microsoft/playwright/blob/v1.61.1/packages/playwright/src/reporters/html.ts), [workflow artifacts](https://docs.github.com/en/actions/concepts/workflows-and-actions/workflow-artifacts)

The 30-minute job timeout cancels the whole job. The failure-upload step cannot guarantee artifacts after cancellation, and Playwright may not have finalized its HTML report. Accept this limitation initially. If diagnostics from suite hangs become necessary, configure a Playwright `globalTimeout` with enough margin below the job deadline for reporting and upload, accounting for provisioning and build time. Changing `failure()` to `always()` alone would not finalize the report or provide more job time. [GitHub job timeout](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#jobsjob_idtimeout-minutes), [Playwright suite timeout](https://playwright.dev/docs/api/class-testconfig#test-config-global-timeout)

Use `ubuntu-24.04-arm` as an explicit OS generation and architecture; its hosted image still receives updates. Standard hosted execution for this public repository is free. Start with a 30-minute job timeout for cold provisioning and adjust after measurement. Nix’s normal binary substitutes and pnpm installation are sufficient initially; the previously inspected packages required about 490 MiB from cache.nixos.org, plus the LFS objects. Remeasure the final shell with the added tools. Add Nix or Astro caching only if measured runtime warrants it. One Linux job already runs both configured browsers. [Hosted runners](https://docs.github.com/en/actions/reference/runners/github-hosted-runners), [runner image updates](https://docs.github.com/en/actions/concepts/runners/github-hosted-runners), [cache behavior](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching)

Use the default PR activities: opened, synchronize, and reopened, including drafts. Preserve checkout’s default test-merge commit so checks cover the change together with its base branch. Changing a PR’s base does not trigger these checks, and the up-to-date rule does not start a run. After changing the base, close and reopen the PR to request fresh checks. Rerunning the old workflow keeps its original commit. Document this rare case rather than running checks on every title or description edit. [Workflow events](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows), [rerun behavior](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/re-run-workflows-and-jobs)

Group runs by PR number so a newer revision cancels its older run. Give each `main` run its own group using `github.run_id`, so every push can be checked. A shared `main` group would discard older waiting runs even with `cancel-in-progress: false`. These checks do not need to run one at a time. [Concurrency](https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/control-workflow-concurrency)

This setup needs only read permission and no repository secrets. Fork PRs can use it, although outside contributors may need maintainer approval under the repository’s Actions settings. Keep full action commit pins and disable checkout credential persistence. [Fork approvals](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/approve-runs-from-forks), [secure use](https://docs.github.com/en/actions/reference/security/secure-use)

Stop Dependabot’s automatic PRs: disable security-update PRs and any configured version updates, while keeping vulnerability alerts enabled. These are separate features; there is no tracked Dependabot configuration in this checkout. Resolve the existing bot PRs explicitly. npm and action updates then become manual. [Security-update settings](https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/secure-your-dependencies/configure-security-updates), [version-update settings](https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/secure-your-dependencies/configure-version-updates)

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

Rulesets are available for this public personal repository. Requiring a PR and requiring a check are separate settings; zero approvals permits solo maintenance. When `main` advances, update the PR branch and rerun checks before merging. An empty bypass list applies the rule to normal owner merges too; an owner with settings access can still edit the policy. “Restrict updates” would limit updates to bypass actors and should remain off. [About rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets), [available rules](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets)

GitHub requires named checks; it does not automatically require every independent check added later. Keep `test` as the stable check representing `pnpm test`, and select the emitted name observed in the actual run, rather than the workflow display name `Checks`. Selecting GitHub Actions as the source identifies the reporting integration; the workflow and scripts remain normal reviewable repository code. [Creating rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository), [check naming](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/troubleshooting-rules)

Keep the validation job unconditional, with no path filters or `continue-on-error`. GitHub accepts skipped or neutral required checks as well as successful ones, so skipping a job can satisfy a requirement. Conversely, filtering out an entire required workflow can leave its check pending. The single unconditional job and explicit script chain avoid these pitfalls. [Required-check behavior](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks)

A merge queue is unnecessary for this repository. The review could not confirm from the documentation whether personal accounts can enable one, and the answer does not change the recommendation. Add `merge_group` only if the repository later adopts a queue. [Merge queue setup](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-a-merge-queue)

Keep the implementation limited to these repository files and GitHub settings:

| Location | Work |
| --- | --- |
| `package.json` | Define the checks; keep `test:visual` as `playwright test`. |
| `flake.nix` | Add `pkgs.actionlint` and `pkgs.git-lfs`; define one fontconfig file that ignores host and user preferences. |
| `src/utils/date.ts` and a nearby unit test | Format calendar dates in UTC and verify timezone independence. |
| `playwright.config.ts` | Add focused-test protection, one CI worker, failure traces, the `github` reporter in CI, and a longer server timeout with piped stdout; preserve build-and-preview startup. |
| `.github/workflows/checks.yaml` | Add the workflow above. |
| `tests/visual.spec.ts-snapshots/` | Update only for understood, reviewed font or host-rendering changes; preserve comparison evidence. |
| `README.md` | Document Nix and LFS setup, testing, baseline authority and review, report viewing, paired Playwright upgrades, and changing a PR’s base. |
| GitHub repository settings | Activate and verify the ruleset; stop Dependabot PRs, retain alerts, and check LFS usage and budget. |

The current lockfiles need no updates merely to use packages from the already pinned nixpkgs input. Existing non-failing Biome schema and unsupported-Markdown notices do not require expanding this change into a lint-policy cleanup. The pnpm build allowlist names `protobufjs` and `re2`, neither of which appears in the lockfile; removing those entries is an optional, unrelated cleanup. [Build allowlist](https://github.com/sullvn/sull.vn/blob/9836e696c3a0c072968296ecc4dbb17a663427ac/pnpm-workspace.yaml)

Keep the README short: give the fresh-clone Nix/LFS/install/test sequence, the baseline-update and saved-report commands, and the supported headless Linux arm64 environment. Explain that Linux snapshot names omit architecture, retain the minimal-font tradeoff, and cover the maintenance rules above. Save these instructions before deleting this temporary proposal.

Roll out and verify the implementation in this order:

1. Implement the repository changes and verify the documented Nix/LFS setup from a fresh clone. Run `pnpm test` in Nix. Remove `node_modules/.astro` and `dist/`, then run `pnpm test:visual` alone to verify the build and preview startup fit the server timeout. Confirm a fresh direct Playwright invocation also builds before testing. Check that extra user fonts and preferences cannot alter rendering, and that date tests pass across timezones. Preserve and review comparisons before accepting baseline changes. Temporarily introduce a type/build error, confirm useful failure output through the piped server log, then restore it.
2. Stop Dependabot PRs and check the owner’s LFS usage and budget. Open the implementation PR and obtain a successful run on a fresh arm64 runner. Verify LFS downloads, frozen installation, all six visual cases, and cold runtime, adjusting the initial server timeout if needed. Follow the mismatch decision table and record whether local generation reproduces the authoritative CI environment. A PR can run the workflow it introduces; no preliminary merge is needed.
3. Select the emitted `test` check and GitHub Actions source, then activate the ruleset. GitHub requires a check to have completed successfully in the repository within the preceding seven days to be selectable.
4. On that same PR, push a deliberate visual failure. Confirm it blocks merging for the owner and produces a usable downloadable report with comparisons and a trace. Fix it and verify that the new run restores mergeability. This also checks that the previous successful result cannot satisfy a newer failing revision. Confirm strict freshness in the active ruleset; when `main` advances, incorporate it and rerun checks.
5. Save the testing and maintenance instructions in the README, remove this proposal, and obtain a final green run. Leave the PR ready for review and merge with enforcement active. After merging, confirm the `main` run passes and record runtime and LFS usage.

Using the same PR avoids depending on a workflow that has not yet reached `main`. Committing YAML and enabling enforcement are distinct operations; a ruleset JSON file in the repository would not apply GitHub settings by itself. [PR workflow execution](https://docs.github.com/en/actions/reference/security/securely-using-pull_request_target), [required-check registration](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks), [ruleset configuration](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository)

The design follows your `shai` and `cowbox` precedents: ordinary repository commands, default PR and default-branch triggers, and releases kept separate. Nix supplies this repository’s tools as requested. [shai test workflow](https://github.com/sullvn/shai/blob/bb3382b83a3e7a7a163e92f286fe095dbe6c0e05/.github/workflows/test.yaml), [cowbox test workflow](https://github.com/sullvn/cowbox/blob/0deff1b6fb796e03a7c0b181dc08cdddb76bdebe/.github/workflows/test.yaml)

At inspected commit `9836e696c3a0c072968296ecc4dbb17a663427ac`, there were no tracked workflows. Public API responses identified a personally owned public repository with `main` unprotected and no rulesets. These observations establish why both code and GitHub settings are needed; no settings have been applied. [Repository metadata](https://api.github.com/repos/sullvn/sull.vn), [rulesets](https://api.github.com/repos/sullvn/sull.vn/rulesets), [main branch](https://api.github.com/repos/sullvn/sull.vn/branches/main)

Earlier validation passed unit tests, `pnpm check`, and all six visual cases in the existing environment. The visual run took 15.1 seconds with cached images; a fresh copy produced the cold build times above. Later review reproduced the user-font override and timezone-dependent dates described here. Package availability in the Nix cache does not prove a fresh shell will start successfully. A fresh runner, the corrected font configuration, and live merge enforcement still need implementation-time validation.

The latest review passed the existing typography test command and read-only Biome check, with the already documented non-failing notices. It reconfirmed the UTC/Los Angeles date difference, inspected the single-face DejaVu package and Linux-only snapshot suffix, and verified Playwright’s existing font wait and reporter behavior. It did not rerun builds or visual tests, modify baselines, or exercise GitHub rulesets. The revised five-minute startup budget and the local/CI baseline agreement remain unverified until implementation.

This revision changes only the proposal. No CI implementation or GitHub settings have been changed. The design assumes `main` is the branch to protect, zero human approvals suit solo maintenance, checks apply to owner merges, and automatic Dependabot PRs are disabled while alerts remain enabled.
