# fpm staging deployment

## Environment

Copy `.env.staging.example` to `.env.staging` on the Docker host and fill in the real PostgreSQL, IdP, JWT, and R2 credentials. Do not commit `.env.staging`. For the first staging run, use the test public object-storage domain `https://test.fuyeor.net` as `R2_PUBLIC_URL_BASE`; production can use `https://fpm.fuyeor.net`.

Apply the existing Prisma migrations before starting the registry. The backend reads the same PostgreSQL schema through SeaORM entities generated from the Prisma source of truth.

## Start the registry

```bash
git checkout feat/fpm-publish-install
git pull --ff-only
cp .env.staging.example .env.staging
# Edit .env.staging with staging-only values.
pnpm install --frozen-lockfile
pnpm -F @fuyeor/prisma-registry prisma migrate deploy
docker compose -f docker-compose.staging.yml up -d --build
curl --fail https://fpm.fuyeor.com/v1/health
```

The container listens on `6011`. The application itself serves `/health`, `/packages/acquire`, `/packages/commit`, and package metadata routes. Nginx should remove the external `/v1` prefix before proxying:

```nginx
location /v1/ {
    proxy_pass http://127.0.0.1:6011/;
    proxy_http_version 1.1;
    proxy_set_header Host $host;
    proxy_set_header X-Request-Id $request_id;
    proxy_set_header X-Forwarded-Proto $scheme;
}
```

The package tarball URL is not proxied through the registry. Metadata points pnpm directly to R2 using this layout:

```text
https://fpm.fuyeor.net/packages/@scope/name/version.tgz
```

## First publish and pnpm acceptance

Create a Personal Access Token in the Web UI, make sure its owner is a member of the target organization scope, and run the CLI from a checked-out fpm/monorepo sibling layout:

```bash
pnpm --filter @fuyeor/fpm-cli build
node packages/cli/dist/cli.js login --registry https://fpm.fuyeor.com/v1
node packages/cli/dist/cli.js publish ../monorepo/packages/commons --dry-run --json
node packages/cli/dist/cli.js publish ../monorepo/packages/commons
```

In a separate clean consumer project, configure public scoped downloads:

```ini
@fuyeor:registry=https://fpm.fuyeor.com/v1
```

Then verify all three required paths:

```bash
pnpm add @fuyeor/commons
pnpm add @fuyeor/commons@1.0.0
pnpm install
```

The expected evidence is a successful metadata response, a successful R2 tarball response, and a package installed under `node_modules/@fuyeor/commons`. Repeat the publish command with the same version to verify that the registry rejects immutable duplicate versions. Use a token from a non-member account to verify scope permission denial, and revoke the publishing token in the Web UI to verify subsequent `acquire` requests return `401`.

## Rollback

Because package versions are immutable, application rollback is performed by switching the registry image to the previous commit and preserving the database and R2 objects. Do not delete published objects during an application rollback.
