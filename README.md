**fpm (Fuyeor Package Manager)** is a JavaScript package manager built for Ф. It prioritizes environmental sustainability, developer sovereignty, and radical simplicity.

[![License: MIT](https://img.shields.io/badge/License-MIT-AEA4E4?style=flat-square)](./LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2024_Edition-AEA4E4?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)

### Build for...

- **Eco-Friendly & Build-Free**  
  `fpm` encourages **Direct-to-Source** publishing, reducing unnecessary build steps and storage waste.

- **Author Sovereignty**  
  We believe authors should have full control over their creations. Authors can unpublish packages at any time, respecting their right to manage the code's lifecycle. This prevents the registry from being cluttered with redundant or obsolete versions.

- **Traditional Simplicity**  
  `fpm` strips away the complexity of modern registry authentication. No complex configuration files or multi-step handshakes are required. Simply use your token and publish instantly.

### Development

**Prerequisites**

```bash
git clone https://github.com/Fuyeor/fpm
cd fpm && pnpm i
```

**Run Front-end**

```bash
pnpm -F @fuyeor/fpm-front-end dev
```

**Run Back-end**

```bash
cd apps/registry/back-end && cargo run
```

### Database Migration

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