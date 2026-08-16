# fpm

[![License: MIT](https://img.shields.io/badge/License-MIT-AEA4E4?style=flat-square)](./LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2024_Edition-AEA4E4?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)

FPM 是面向 JavaScript/TypeScript package 的公开 registry。它保持源码直接发布的定位，package 作者发布原始源码与 manifest，消费者通过 pnpm 使用 npm-compatible metadata 和公开 tarball 下载 package。

## Build for...

- **Eco-Friendly & Build-Free**  
  `fpm` encourages **Direct-to-Source** publishing, reducing unnecessary build steps and storage waste.

- **Author Sovereignty**  
  We believe authors should have full control over their creations. Authors can unpublish packages at any time, respecting their right to manage the code's lifecycle. This prevents the registry from being cluttered with redundant or obsolete versions.

- **Traditional Simplicity**  
  `fpm` strips away the complexity of modern registry authentication. No complex configuration files or multi-step handshakes are required. Simply use your token and publish instantly.

## Development

**Prerequisites**

The repository targets the latest supported Node.js LTS line and the latest pnpm release. The CLI declares Node.js `>=24.19.0`; older Node.js and pnpm versions are not supported.

```bash
git clone https://github.com/Fuyeor/fpm
cd fpm
pnpm install
```

**Run Front-end**

```bash
pnpm -F @fuyeor/fpm-front-end dev
```

**Run Back-end**

```bash
cd apps/registry/back-end && cargo run
```

The backend listens on port `6011`. In deployment, nginx exposes it under the `/v1` prefix, so the public health endpoint is:

```bash
curl https://fpm.fuyeor.com/v1/health
```

## Publish packages

The independent `@fuyeor/fpm-cli` package publishes source packages through the registry's two-phase API. It does not use the npm legacy `PUT /package` protocol and does not execute unknown lifecycle scripts. The packer reads `package.json`, honors the manifest `files` field, applies the package-local `.gitignore`, and always excludes `.git`, `node_modules`, and temporary files.

Build and use the CLI from this repository:

```bash
pnpm --filter @fuyeor/fpm-cli build
node packages/cli/dist/cli.js login --registry https://fpm.fuyeor.com/v1
node packages/cli/dist/cli.js publish ../monorepo/packages/commons
```

For CI, provide the token through `FPM_TOKEN` and optionally override the endpoint with `FPM_REGISTRY`. The CLI never prints the token and stores interactive credentials under the platform user configuration directory with restrictive permissions. Use `--dry-run` before publishing a package:

```bash
FPM_TOKEN=fpm_... \
  node packages/cli/dist/cli.js publish ../monorepo/packages/commons --dry-run --json
```

Publishing requires a valid `fpm_...` Personal Access Token and membership in the organization represented by the package scope. A package version is immutable: the same package name and SemVer version cannot be published twice.

## Install packages through pnpm

FPM downloads are public. Configure a scope-specific registry in the consumer project or workspace:

```ini
@fuyeor:registry=https://fpm.fuyeor.com/v1
```

Then use standard pnpm commands:

```bash
pnpm add @fuyeor/commons
pnpm add @fuyeor/commons@1.0.0
pnpm install
```

The registry supports npm abbreviated installation metadata, the `latest` dist-tag derived from the highest valid SemVer, and public tarballs under this stable layout:

```text
https://fpm.fuyeor.net/packages/@scope/name/version.tgz
```

No authentication entry is required for public downloads. Publishing and future package access-control settings remain separate from anonymous installation.

## Database Migration

> We utilize Prisma for database migrations because its schema syntax provides a highly readable, declarative view of the database state, serving as the Single Source of Truth for the entire project.

**1. Create a Migration**

First, modify the [schema.prisma](./packages/prisma/schema.prisma) file to reflect your changes, then run:

```bash
# Generate a new migration file without applying it
pnpm -F @fuyeor/prisma-registry prisma migrate dev --create-only --name [migration_name]
```

**2. Review and Deploy**

Carefully review the generated SQL file in the `migrations` directory. Once verified, apply the changes to the database:

```bash
# Apply migrations to the database
pnpm -F @fuyeor/prisma-registry prisma migrate deploy
```

**3. Synchronize Backend (Sea-ORM)**

After the database schema is updated, synchronize the Rust entities for the backend:

```bash
# Generate Rust entities for Sea-ORM
pnpm -F @fuyeor/prisma-registry generate:rust
```
