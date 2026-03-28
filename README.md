# Quilix

A React framework powered by Rust and Rspack.

## Features

- **Rust CLI** — Fast, native CLI (`quilix dev`, `quilix build`, `quilix preview`)
- **Rspack** — Blazing fast bundling
- **Module Federation** — Built-in micro-frontend support via native Rspack container
- **App Router** — File-based routing conventions
- **Biome** — Integrated formatting and linting
- **Tailwind CSS** — First-class Tailwind v4 support

## Prerequisites

- Node.js 18+
- Rust 1.85+
- pnpm 8+ (recommended), npm, yarn, or bun

## Quick Start

```bash
# Create a new project
npx create-quilix-app my-app
cd my-app
pnpm install
pnpm dev
```

## CLI Commands

```bash
quilix dev              # Start dev server with HMR
quilix build            # Production build
quilix preview          # Preview production build
quilix lint             # Run Biome linter
quilix lint --fix       # Auto-fix lint issues
quilix generate page    # Generate a new page
quilix generate component # Generate a new component
quilix generate api     # Generate an API route
quilix add tailwind     # Add Tailwind CSS
quilix add module-federation  # Add Module Federation
```

## Module Federation

### Host App

```typescript
// quilix.config.ts
import { defineConfig, moduleFederation } from '@quilix/core';

export default defineConfig({
  plugins: [
    moduleFederation({
      name: 'host',
      remotes: {
        remote: 'remote@http://localhost:3001/remote.js',
      },
      shared: ['react', 'react-dom'],
    }),
  ],
});
```

### Remote App

```typescript
// quilix.config.ts
import { defineConfig, moduleFederation } from '@quilix/core';

export default defineConfig({
  plugins: [
    moduleFederation({
      name: 'remote',
      exposes: {
        './Button': './app/components/Button.tsx',
      },
      shared: ['react', 'react-dom'],
    }),
  ],
});
```

### Loading Remote Components

```typescript
// Dynamically import from remote
const { default: RemoteButton } = await import('remote/Button');
```

## Local Development

```bash
# Install dependencies
cargo build --workspace
pnpm install

# Run CLI
cargo run --package quilix-cli -- dev

# Run examples
cd examples/basic && pnpm install && cargo run --package quilix-cli -- dev
cd examples/micro-fe && pnpm install
```

## Code Quality

```bash
cargo fmt                  # Format Rust code
cargo clippy --workspace --all-features -- -D warnings  # Lint Rust
pnpm lint                  # Lint TypeScript
```

## Project Structure

```
quilix/
├── crates/
│   ├── quilix-cli/        # CLI commands
│   ├── quilix-config/     # Config parsing
│   ├── quilix-mf/         # Module Federation
│   ├── quilix-router/     # Routing
│   └── quilix-utils/      # Utilities
├── packages/
│   ├── @quilix/core/      # React components
│   └── @quilix/create-app/ # Scaffolding
└── examples/
    ├── basic/             # Basic app
    └── micro-fe/          # Module Federation demo
```

## License

MIT
