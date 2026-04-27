-- Enable CITEXT for case-insensitive usernames
CREATE EXTENSION IF NOT EXISTS "citext";

-- Users table, synced from IdP. Read-only from FPM's perspective.
CREATE TABLE "user" (
    "id" UUID PRIMARY KEY,
    "username" CITEXT UNIQUE NOT NULL,
    "nickname" VARCHAR(64) NOT NULL,
    "avatar" VARCHAR(255),
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Organizations (scopes like @webroamer)
CREATE TABLE "organization" (
    "id" UUID PRIMARY KEY,
    "name" CITEXT UNIQUE NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Many-to-Many relationship between users and organizations
CREATE TABLE "organization_member" (
    "organization_id" UUID NOT NULL REFERENCES "organization"("id") ON DELETE CASCADE,
    "user_id" UUID NOT NULL REFERENCES "user"("id") ON DELETE CASCADE,
    "role" VARCHAR(32) NOT NULL, -- e.g., 'admin', 'publisher'
    PRIMARY KEY ("organization_id", "user_id")
);

-- Access Tokens for CLI, storing only the SHA-256 hash
CREATE TABLE "token" (
    "id" UUID PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES "user"("id") ON DELETE CASCADE,
    "name" VARCHAR(64) NOT NULL,
    "token_hash" VARCHAR(64) UNIQUE NOT NULL, -- SHA-256 hex string
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Packages table
CREATE TABLE "package" (
    "id" UUID PRIMARY KEY,
    "organization_id" UUID NOT NULL REFERENCES "organization"("id") ON DELETE CASCADE,
    "name" VARCHAR(255) NOT NULL, -- e.g., "core"
    "full_name" CITEXT UNIQUE NOT NULL, -- e.g., "@webroamer/core"
    "description" TEXT,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Package Versions table
CREATE TABLE "package_version" (
    "id" UUID PRIMARY KEY,
    "package_id" UUID NOT NULL REFERENCES "package"("id") ON DELETE CASCADE,
    "version" VARCHAR(64) NOT NULL, -- e.g., "1.0.0"
    "manifest" JSONB NOT NULL, -- The entire package.json content
    "dist_tarball" VARCHAR(255) NOT NULL, -- Local path or URL to .tar.gz
    "dist_shasum" VARCHAR(128) NOT NULL, -- Integrity check
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE("package_id", "version")
);