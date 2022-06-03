# Releasing

## Creating a New Version

Run `./scripts/version.sh <NEW_VERSION>`. This will update the version in [Cargo.toml](./Cargo.toml) and [npm/package.json](./npm/package.json) to the one passed in.

## Releasing a New Version

Run `./scripts/tag.sh`. This will get the version in [Cargo.toml](./Cargo.toml), create a tag, and push it.
