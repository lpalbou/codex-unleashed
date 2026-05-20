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

Create a GitHub environment named `npm` and restrict it to the `main` branch.
Add required reviewers if desired.

Configure npm trusted publishing for the package:

```bash
npm install -g npm@latest
npm trust github @lpalbou/codex-unleashed \
  --repo lpalbou/codex \
  --file npm-publish-codex-unleashed.yml \
  --env npm
```

The equivalent npmjs.com settings are:

- Package: `@lpalbou/codex-unleashed`
- Publisher: GitHub Actions
- Organization/user: `lpalbou`
- Repository: `codex`
- Workflow filename: `npm-publish-codex-unleashed.yml`
- Environment: `npm`
- Allowed action: `npm publish`

The npm trust command requires the package to already exist on npm. If this is
the first publication, manually publish one reviewed package version from the
validated tarballs, then configure trusted publishing and remove any temporary
publish token.

## Release flow

1. Build native release artifacts from reviewed `main` code with the existing
   `rust-release` workflow. The release tag must point at a commit on `main`.
2. Run `npm-publish-codex-unleashed` from `main` with `publish=false` and the
   successful `rust-release` workflow URL.
3. Review the uploaded `codex-unleashed-npm-<version>` artifact.
4. Run the same workflow again with `publish=true` and
   `publish_confirmation=publish-lpalbou-codex-unleashed-<version>`.

The publish job stages seven npm tarballs: the lightweight
`@lpalbou/codex-unleashed` meta package plus six platform payload versions. It
publishes platform payloads first under platform dist-tags, then publishes the
meta package under `latest`, `alpha`, or `beta`.
