# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-26

### Added

- Initial release
- Rust CLI with commands: `dev`, `build`, `preview`, `lint`, `generate`, `add`
- `@quilix/core` package with React components (Link, Layout)
- `@quilix/rspack-plugin` for Rspack integration
- `@quilix/create-app` for project scaffolding
- Dev server with HMR support via WebSocket
- Module Federation support (host/remote modes)
- File-based routing (Next.js App Router conventions)
- Biome for linting and formatting
- Tailwind CSS support
- GitHub Actions CI workflow
- GitHub Actions release workflow for npm and crates.io publishing
