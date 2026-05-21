# npm publishing

Codex Unleashed publishes to the npm scope `@lpalbou` as
`@lpalbou/codex-unleashed`. The installed executable remains
`codex-unleashed`.

```bash
npm install -g @lpalbou/codex-unleashed
```

Publishing is intentionally manual and restricted to `main` through
`.github/workflows/npm-publish-codex-unleashed.yml`. Do not push private local
work branches to use this workflow.

## One-time setup

Create a GitHub environment named `npm` and restrict publishing to the `main`
branch. Add required reviewers if desired.

The repository can be configured from a machine authenticated with `gh`:

```bash
gh api \
  --method PUT \
  -H "Accept: application/vnd.github+json" \
  /repos/lpalbou/codex-unleashed/environments/npm \
  -F deployment_branch_policy[protected_branches]=false \
  -F deployment_branch_policy[custom_branch_policies]=true

gh api \
  --method POST \
  -H "Accept: application/vnd.github+json" \
  /repos/lpalbou/codex-unleashed/environments/npm/deployment-branch-policies \
  -f name=main
```

GitHub Pages is built by `.github/workflows/docs.yml`. Enable Pages with GitHub
Actions as the build source:

```bash
gh api \
  --method POST \
  -H "Accept: application/vnd.github+json" \
  /repos/lpalbou/codex-unleashed/pages \
  -f build_type=workflow
```

Configure npm trusted publishing for the package after the package exists:

```bash
npm install -g npm@^11.10.0
npm --version
npm trust github @lpalbou/codex-unleashed \
  --repo lpalbou/codex-unleashed \
  --file npm-publish-codex-unleashed.yml \
  --env npm
```

The equivalent npmjs.com settings are:

- Package: `@lpalbou/codex-unleashed`
- Publisher: GitHub Actions
- Organization/user: `lpalbou`
- Repository: `codex-unleashed`
- Workflow filename: `npm-publish-codex-unleashed.yml`
- Environment: `npm`
- Allowed action: `npm publish`

The npm trust command requires the package to already exist on npm. For the
first publication, run the workflow with `publish=false`, download the validated
tarballs, publish them manually from an npm-authenticated machine, then
configure trusted publishing for future releases.

## Release flow

1. Run `native-artifacts` from `main`.
2. Copy the successful `native-artifacts` workflow run URL.
3. Run `npm-publish-codex-unleashed` from `main` with `publish=false`,
   `version=<package version>`, and the native artifact workflow URL.
4. Review the uploaded `codex-unleashed-npm-<version>` artifact.
5. For the first npm publication only, download the artifact and publish each
   tarball manually with `npm publish --access public`.
6. Configure npm trusted publishing.
7. For later releases, run the same workflow with `publish=true` and
   `publish_confirmation=publish-lpalbou-codex-unleashed-<version>`.

The publish job currently stages six npm tarballs: the lightweight
`@lpalbou/codex-unleashed` meta package plus five platform-specific versions of
the same package. Linux ARM64 is deferred until the native artifact build is
stable. The workflow publishes platform payloads first under platform dist-tags,
then publishes the meta package under `latest`, `alpha`, or `beta`.

## Commands

Trigger native artifacts:

```bash
gh workflow run native-artifacts.yml \
  --repo lpalbou/codex-unleashed \
  --ref main
```

Watch the run and capture its URL:

```bash
gh run list \
  --repo lpalbou/codex-unleashed \
  --workflow native-artifacts.yml \
  --limit 1
```

Stage npm tarballs without publishing:

```bash
gh workflow run npm-publish-codex-unleashed.yml \
  --repo lpalbou/codex-unleashed \
  --ref main \
  -f version=0.1.0 \
  -f native_artifacts_workflow_url=https://github.com/lpalbou/codex-unleashed/actions/runs/<run-id> \
  -f publish=false
```

Download the staged tarballs from the stage run:

```bash
gh run download <stage-run-id> \
  --repo lpalbou/codex-unleashed \
  --name codex-unleashed-npm-0.1.0 \
  --dir dist/npm
```

First npm publication only:

```bash
npm login

npm publish dist/npm/codex-npm-linux-x64-0.1.0.tgz --access public --tag linux-x64
npm publish dist/npm/codex-npm-darwin-x64-0.1.0.tgz --access public --tag darwin-x64
npm publish dist/npm/codex-npm-darwin-arm64-0.1.0.tgz --access public --tag darwin-arm64
npm publish dist/npm/codex-npm-win32-x64-0.1.0.tgz --access public --tag win32-x64
npm publish dist/npm/codex-npm-win32-arm64-0.1.0.tgz --access public --tag win32-arm64
npm publish dist/npm/codex-npm-0.1.0.tgz --access public
```

The final meta package publish is what makes `npm install -g
@lpalbou/codex-unleashed` resolve the install launcher. If the platform payloads
exist but `npm view @lpalbou/codex-unleashed@0.1.0 version` returns `E404`,
publish only `dist/npm/codex-npm-0.1.0.tgz` and complete the npm 2FA/browser
approval prompt.

After trusted publishing is configured, publish:

```bash
gh workflow run npm-publish-codex-unleashed.yml \
  --repo lpalbou/codex-unleashed \
  --ref main \
  -f version=0.1.0 \
  -f native_artifacts_workflow_url=https://github.com/lpalbou/codex-unleashed/actions/runs/<run-id> \
  -f publish=true \
  -f publish_confirmation=publish-lpalbou-codex-unleashed-0.1.0
```
