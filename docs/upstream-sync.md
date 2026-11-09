# Upstream mirror credentials

The scheduled sync copies the upstream commit unchanged to `upstream-main`.
Upstream commits can change `.github/workflows`, which the built-in
`GITHUB_TOKEN` cannot push even with `contents: write`.

The sync uses a dedicated SSH write deploy key belonging only to this repository.
Register its public key under **Settings > Deploy keys**, with **Allow write
access** enabled, and store its private key in the repository Actions secret
`UPSTREAM_SYNC_SSH_KEY`. Do not reuse the key in other repositories. The workflow
keeps the built-in token read-only and reports a missing secret before checkout.

To rotate the credential, generate a new key pair, register the new public key,
replace the Actions secret, and run **Sync upstream mirror** manually. Verify
that `upstream-main` matches `rust-windowing/winit`'s `master` before removing
the old deploy key. Delete temporary private key files after storing the secret.

The job updates only `upstream-main`, with `--force-with-lease` against the
observed remote head; it leaves `main` unchanged. A deploy key is scoped to this
repository, but GitHub does not restrict it to the mirror branch. Keep this
workflow trusted and review repository rules and upstream workflows: a deploy
key push can trigger workflows on the mirrored commit.

For a workflow-only fix, dispatch **Sync upstream mirror** on the repair branch
before merging. The workflow still checks out `main` as its Git workspace;
the selected branch supplies the job definition and credentials configuration.
