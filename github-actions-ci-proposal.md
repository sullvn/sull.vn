**GitHub Actions CI proposal for sullvn/sull.vn**

Reviewed 8 September 2026 · Proposal only · Temporary review document; remove before merging

I recommend one required GitHub Actions job, `test`, which installs dependencies and runs `pnpm test` inside the repository’s locked Nix development shell. Package scripts own the checks; an active branch ruleset requires their successful completion before merging into `main`. Keep the existing browser and font environment initially. Investigate a shared container later if reproducing screenshots requires substantial font configuration.

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

This is the relevant portion of `scripts`; preserve the other development and formatting commands. Retire `check`, whose operations now have named entries under `test:`. There are no current repository callers that need its old name.

Diagnostics run once through `test → test:visual → build → test:types`; the production build also runs once. This preserves the existing guarantee that `pnpm build` performs diagnostics, and keeps `pnpm test:visual` independently runnable against a fresh build. `pnpm test:types` remains available for fast, targeted feedback. The inspected Astro checker includes the repository’s TypeScript files and Astro components, so another TypeScript checker would duplicate current coverage.

The explicit `&&` chain stops on failure and returns nonzero. A successful aggregate means every command completed successfully. Keep that list explicit when adding checks; a wildcard selector could unintentionally include a future snapshot-update or interactive script. A custom runner, composite action, or reusable workflow is unnecessary for this single job. [pnpm scripts](https://pnpm.io/cli/run), [GitHub’s guidance on repository scripts](https://docs.github.com/en/actions/how-tos/write-workflows/choose-what-workflows-do/add-scripts)

`actionlint` is the one additional checker: this change introduces workflow YAML, and actionlint understands its syntax, expressions, and job dependencies. Add `pkgs.actionlint` to the existing development shell and run it alongside Biome in `test:lint`. The locked nixpkgs package supplies its ShellCheck and Pyflakes integrations. This adds a normal command, with no additional GitHub Action. [actionlint checks](https://github.com/rhysd/actionlint/blob/main/docs/checks.md), [locked actionlint package](https://github.com/NixOS/nixpkgs/blob/42f17a57f4f6e33b3de3dca0a2a5ea5233169d02/pkgs/by-name/ac/actionlint/package.nix)

The Nix shell remains the tool definition. The committed lockfile selects Node 26.8.1, pnpm 11.25.0, Biome 2.5.11, and Playwright’s browser package at 1.61.1; the npm manifest pins `@playwright/test` to 1.61.1. Preserve the existing browser path, download suppression, and Nix host-validation settings. Update the npm Playwright dependency and Nix browser package together when upgrading. [Repository flake](https://github.com/sullvn/sull.vn/blob/9836e696c3a0c072968296ecc4dbb17a663427ac/flake.nix), [locked Playwright package](https://github.com/NixOS/nixpkgs/blob/42f17a57f4f6e33b3de3dca0a2a5ea5233169d02/pkgs/development/web/playwright/driver.nix)

Use `nix develop --no-update-lock-file` and `pnpm install --frozen-lockfile` to reject dependency definitions that would require lockfile changes. Install the normal development dependencies with the existing package-build allowlist. Nix supplies tools and browsers; pnpm supplies the application’s JavaScript dependencies. Keep `pkgs.chromium` too: the visual suite uses Playwright’s browser bundle, but the repository’s MCP configuration uses that separate executable. [Nix develop reference](https://nix.dev/manual/nix/2.34/command-ref/new-cli/nix3-develop), [pnpm installation](https://pnpm.io/cli/install)

Screenshot portability is the main remaining uncertainty. The site uses system-font stacks, and a `linux` snapshot name does not distinguish distributions or host font settings. Playwright recommends generating and comparing baselines in the same environment. The locked Nix font helper can also include host font directories and `/etc/fonts/conf.d`, so pinned browser binaries alone do not establish identical rendering. This is a plausible source of differences, not an observed failure on the proposed Ubuntu runner. [Playwright visual comparisons](https://playwright.dev/docs/test-snapshots), [locked font helper](https://github.com/NixOS/nixpkgs/blob/42f17a57f4f6e33b3de3dca0a2a5ea5233169d02/pkgs/development/libraries/fontconfig/make-fonts-conf.nix)

Use the first Ubuntu run to decide whether font work is necessary:

| Result with the current browser setup and snapshots | Action |
| --- | --- |
| All six cases pass | Keep the current font environment. |
| A mismatch has a small, demonstrated font fix | Apply it through Nix and verify it locally and on Ubuntu. |
| Matching requires custom fallback tuning, browser-specific overrides, or additional baseline sets | Defer that work to a consistent visual test environment. |

A shared container is a reasonable follow-up for the last case: baseline generation and comparison could share pinned browsers, fonts, and rendering libraries. Keep its image design and browser/preview wiring outside this change. The future target should be the same image and architecture locally and in CI, with matching Playwright versions; containerization alone does not guarantee identical rendering on every host. [Playwright containers](https://playwright.dev/docs/docker), [rendering variability](https://playwright.dev/docs/test-snapshots)

Keep visual tests required throughout. If the environment needs substantial work, completion of the full merge requirement remains pending that follow-up or an explicit scope decision. Review baseline changes on their merits and preserve the existing comparison tolerances.

Limit Playwright configuration changes to four useful settings:

| Setting | Value | Purpose |
| --- | --- | --- |
| `forbidOnly` | `!!process.env.CI` | Reject accidentally committed focused tests in CI. |
| `workers` | `process.env.CI ? 1 : undefined` | Follow Playwright’s CI recommendation for consistent resources across this small suite. |
| `use.trace` | `'retain-on-failure'` | Preserve a trace for diagnosing a failed visual case. |
| `webServer.stdout` | `'pipe'` | Show Astro’s type-check and build diagnostics in the job log. |

Playwright normally ignores server stdout, while Astro writes detailed diagnostics there. Piping it makes failed type checks and builds understandable in the logs. Preserve the existing preview command, foreground-process environment setting, `reuseExistingServer: false`, and all six projects. Measure startup before increasing the server timeout. [Playwright CI guidance](https://playwright.dev/docs/ci), [server output and lifecycle](https://playwright.dev/docs/test-webserver), [recording options](https://playwright.dev/docs/test-use-options#recording-options)

Keep the existing HTML reporter and the default retry and snapshot settings. In installed Playwright 1.61.1, HTML report opening is already suppressed when `CI` is set, a console reporter is added when necessary, and retries default to zero. A missing screenshot baseline already fails its test, even though Playwright writes an initial image; an extra `updateSnapshots: 'none'` override is not needed to enforce failure. GitHub supplies `CI=true`, which the normal Nix command preserves. [Pinned HTML reporter](https://github.com/microsoft/playwright/blob/v1.61.1/packages/playwright/src/reporters/html.ts), [pinned reporter selection](https://github.com/microsoft/playwright/blob/v1.61.1/packages/playwright/src/runner/reporters.ts), [configuration defaults](https://playwright.dev/docs/api/class-testconfig), [missing snapshots](https://playwright.dev/docs/test-snapshots), [GitHub variables](https://docs.github.com/en/actions/reference/workflows-and-actions/variables)

The workflow only provisions dependencies, invokes the aggregate command, and preserves failure diagnostics:

```yaml
name: Checks

on:
  pull_request:
    types: [opened, synchronize, reopened, edited]
  push:
    branches: [main]

permissions:
  contents: read

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  test:
    runs-on: ubuntu-24.04
    timeout-minutes: 30
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
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

The three full-SHA pins were verified against [checkout v7.0.1](https://github.com/actions/checkout/releases/tag/v7.0.1), [install-nix-action v31.11.1](https://github.com/cachix/install-nix-action/releases/tag/v31.11.1), and [upload-artifact v7.0.1](https://github.com/actions/upload-artifact/releases/tag/v7.0.1). Keep adjacent version comments when updating them.

`cachix/install-nix-action` installs upstream Nix and enables flakes and `nix-command`. The pinned action already passes GitHub’s ephemeral token to its installer, so an explicit `github_access_token` input is redundant. It needs no Cachix account. Checkout and artifact upload are GitHub-owned; the Nix bootstrap is the only third-party action. [Pinned action definition](https://github.com/cachix/install-nix-action/blob/13d8dd58da0234aa297dedd986986ccb8e7f3e24/action.yml), [pinned installer](https://github.com/cachix/install-nix-action/blob/13d8dd58da0234aa297dedd986986ccb8e7f3e24/install-nix.sh)

Upload the HTML report directory on failure. Playwright copies its screenshot and trace attachments into that report, so uploading the raw results directory as well is unnecessary for normal test failures. Ignoring a missing report lets an earlier lint or installation failure stand on its own. Failures before report generation still have their job logs. [HTML report attachment handling](https://github.com/microsoft/playwright/blob/v1.61.1/packages/playwright/src/reporters/html.ts), [workflow artifacts](https://docs.github.com/en/actions/concepts/workflows-and-actions/workflow-artifacts)

Use `ubuntu-24.04` as an explicit OS generation; its hosted image still receives updates. Standard hosted execution for this public repository is free. Start with a 30-minute job timeout for cold provisioning and adjust after measurement. Nix’s normal binary substitutes and pnpm installation are sufficient initially; add caching only if measured runtime warrants it. One Linux job already runs both configured browsers. [Hosted runners](https://docs.github.com/en/actions/reference/runners/github-hosted-runners), [runner image updates](https://docs.github.com/en/actions/concepts/runners/github-hosted-runners), [cache behavior](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching)

Use `pull_request` for opened, synchronize, reopened, and edited activities, including draft PRs. The explicit `edited` event covers changing a PR’s target branch; the default events omit it. It also reruns checks for title and description edits, a modest tradeoff that keeps the job unconditional. Preserve checkout’s default test-merge commit so checks cover the proposed change together with its current base. [Workflow events](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows), [edited event’s base-branch changes](https://github.com/octokit/webhooks/blob/main/payload-schemas/api.github.com/pull_request/edited.schema.json)

Pushes to `main` validate its latest state after merging. Per-workflow, per-ref concurrency cancels superseded runs for both PRs and `main`; a rapid sequence of merges may therefore cancel an intermediate `main` run. [Concurrency](https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/control-workflow-concurrency)

This setup needs only read permission and no repository secrets. Fork and Dependabot PRs can use it, although outside contributors may require maintainer approval under the repository’s Actions settings. Full action pins and disabled credential persistence keep execution dependencies explicit. [Fork approvals](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/approve-runs-from-forks), [secure use](https://docs.github.com/en/actions/reference/security/secure-use)

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

Rulesets are available for this public personal repository. Requiring a PR and requiring a check are separate settings; zero approvals permits solo maintenance. Strict freshness requires incorporating new changes from `main` and rerunning checks before merging. An empty bypass list applies the rule to normal owner merges too; an owner with settings access can still edit the policy. “Restrict updates” would limit updates to bypass actors and should remain off. [About rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets), [available rules](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets)

GitHub requires named checks; it does not automatically require every independent check added later. Keep `test` as the stable check representing `pnpm test`, and select the emitted name observed in the actual run, rather than the workflow display name `Checks`. Selecting GitHub Actions as the source identifies the reporting integration; the workflow and scripts remain normal reviewable repository code. [Creating rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository), [check naming](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/troubleshooting-rules)

Keep the validation job unconditional, with no path filters or `continue-on-error`. GitHub accepts skipped or neutral required checks as well as successful ones, so skipping a job can satisfy a requirement. Conversely, filtering out an entire required workflow can leave its check pending. The single unconditional job and explicit script chain avoid these pitfalls. [Required-check behavior](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks)

A merge queue is unnecessary for this repository and is not documented as available for personal accounts. Add `merge_group` only if the repository later moves to an eligible organization and adopts a queue. [Merge queue availability and setup](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-a-merge-queue)

Keep the implementation limited to these four repository files and GitHub settings:

| Location | Work |
| --- | --- |
| `package.json` | Establish the aggregate and named checks while preserving build diagnostics. |
| `flake.nix` | Add `pkgs.actionlint`. |
| `playwright.config.ts` | Add focused-test protection, one CI worker, failure traces, and server stdout. |
| `.github/workflows/checks.yaml` | Add the workflow above. |
| GitHub repository settings | Activate and verify the required-check ruleset. |

The current lockfiles need no updates merely to add a package from the already pinned nixpkgs input. Existing non-failing Biome schema and unsupported-Markdown notices do not require expanding this change into a lint-policy cleanup. Font work and baseline changes are conditional on actual findings.

Roll out and verify the implementation in this order:

1. Implement the repository changes and run the complete command locally in Nix. Temporarily introduce a type/build error, verify that it fails the aggregate and prints useful diagnostics, then restore it. The standard lint and unit commands need no separate artificial-failure exercises.
2. Open the implementation PR and obtain a successful run on a fresh Ubuntu runner. Check frozen installation, both browsers, all six visual cases, unchanged baselines, and cold runtime. Apply the font decision above if screenshots differ. A PR can run the workflow it introduces; a preliminary unprotected merge is unnecessary.
3. Select the emitted `test` check and GitHub Actions source, then activate the ruleset. GitHub requires a check to have completed successfully in the repository within the preceding seven days to be selectable.
4. On that same PR, push a deliberate visual failure. Confirm it blocks merging for the owner and produces a usable downloadable report with comparisons and a trace. Fix it and verify that the new run restores mergeability. This also checks that the previous successful result cannot satisfy a newer failing revision. Confirm strict freshness in the active ruleset; when `main` advances, incorporate it and rerun checks.
5. Remove the temporary proposal, obtain a final green run, and leave the implementation PR ready for review and merge with enforcement active. After the normal merge, confirm that the latest state of `main` has a green run and record the measured runtime.

Using the same PR avoids depending on a workflow that has not yet reached `main`. Committing YAML and enabling enforcement are distinct operations; a ruleset JSON file in the repository would not apply GitHub settings by itself. [PR workflow execution](https://docs.github.com/en/actions/reference/security/securely-using-pull_request_target), [required-check registration](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks), [ruleset configuration](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository)

The design follows your `shai` and `cowbox` precedents: ordinary repository commands, PR and default-branch checks, and releases kept separate. Nix supplies this repository’s tools as requested. [shai test workflow](https://github.com/sullvn/shai/blob/bb3382b83a3e7a7a163e92f286fe095dbe6c0e05/.github/workflows/test.yaml), [cowbox test workflow](https://github.com/sullvn/cowbox/blob/0deff1b6fb796e03a7c0b181dc08cdddb76bdebe/.github/workflows/test.yaml)

Research covered all 200 articles in the current GitHub.com Actions index, plus ruleset and tool documentation. Subsequent reviews checked the installed tools, script coverage, PR lifecycle, and enforcement sequence. Enterprise Server variants and external linked pages were outside the full-index review. [GitHub Actions documentation](https://docs.github.com/en/actions)

At inspected commit `9836e696c3a0c072968296ecc4dbb17a663427ac`, there were no tracked workflows. Public API responses identified a personally owned public repository with `main` unprotected and no rulesets. These observations establish why both code and GitHub settings are needed; no settings have been applied. [Repository metadata](https://api.github.com/repos/sullvn/sull.vn), [rulesets](https://api.github.com/repos/sullvn/sull.vn/rulesets), [main branch](https://api.github.com/repos/sullvn/sull.vn/branches/main)

Validation already performed during the initial research: unit tests passed; `pnpm check` passed with zero Astro errors/warnings, four Biome unsupported-Markdown warnings, and one schema-version notice. All six visual cases passed in 15.1 seconds, including build and preview, in the existing development environment. Locked tool versions were evaluated separately. A fresh offline shell attempt reported missing derivations, so it did not establish a clean Nix bootstrap. Fresh Ubuntu execution and actual merge enforcement still require implementation-time validation.

This revision adds only the proposal document to the repository. Its script and workflow examples were checked structurally; no CI implementation or GitHub settings have been changed. A rendered Markdown preview was unavailable. The design assumes `main` is the branch to protect, zero human approvals suit solo maintenance, and required checks apply to owner merges as well.
