DROP INDEX "organization_name_key";

ALTER TABLE "organization" RENAME COLUMN "name" TO "username";

ALTER TABLE "organization" ALTER COLUMN "username" TYPE CITEXT;

ALTER TABLE "organization" ADD COLUMN "description" VARCHAR(2000);

ALTER TABLE "organization_member" ADD COLUMN "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP;

CREATE UNIQUE INDEX "organization_username_key" ON "organization"("username");
