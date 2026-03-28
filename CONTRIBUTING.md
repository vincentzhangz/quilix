# Contributing to Quilix

We welcome contributions! Here's how to get started.

## Development Setup

```bash
# Clone the repository
git clone https://github.com/vincentzhangz/quilix.git
cd quilix

# Install Rust dependencies
cargo build --workspace

# Install Node dependencies
pnpm install
```

## Building

```bash
# Build all Rust crates
cargo build --workspace

# Build CLI release binary
cargo build --release --package quilix-cli

# Run tests
cargo test --workspace

# Run clippy
cargo clippy --all-features -- -D warnings
```

## Project Structure

- `crates/` — Rust crates (CLI, config, dev server, etc.)
- `packages/` — npm packages (`@quilix/core`, `@quilix/rspack-plugin`)
- `examples/` — Example projects

## Code Style

- Rust: Follow `cargo clippy` recommendations
- TypeScript: Run `pnpm lint` to check with Biome
- Use `cargo fmt` for Rust formatting

## Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Run tests and clippy
5. Submit a pull request

## Testing

```bash
# Run all Rust tests
cargo test --workspace

# Run specific crate tests
cargo test --package quilix-mf
```

## Commit Messages

Follow Conventional Commits:

- `feat:` — New feature
- `fix:` — Bug fix
- `docs:` — Documentation
- `refactor:` — Code refactoring
- `test:` — Adding tests

## Release Process

1. Update version in `packages/@quilix/*/package.json` and `Cargo.toml`
2. Update `CHANGELOG.md` with changes
3. Create a git tag: `git tag v0.1.0`
4. Push: `git push origin main --tags`
5. GitHub Actions will automatically publish to npm and crates.io
