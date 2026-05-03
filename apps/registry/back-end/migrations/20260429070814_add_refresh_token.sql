-- User refresh token for session rotation
CREATE TABLE "user_refresh_token" (
    "id" UUID PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES "user"("id") ON DELETE CASCADE,
    "token" VARCHAR(512) UNIQUE NOT NULL,
    "expires_at" TIMESTAMPTZ NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "user_agent" TEXT,
    "ip_address" VARCHAR(45) -- Supports IPv6
);

-- Index for faster lookup by user
CREATE INDEX "idx_user_refresh_token_user_id" ON "user_refresh_token"("user_id");