# Module Federation Example

This example demonstrates Module Federation with a host and remote app.

## Structure

- `host/` - Shell application that loads remote modules
- `remote/` - Exposes the Button component via Module Federation

## Running

Start the remote first:
```bash
cd remote
pnpm install
pnpm dev
```

Then start the host:
```bash
cd host
pnpm install
pnpm dev
```

The host runs on `http://localhost:3000` and the remote on `http://localhost:3001`.

## How It Works

The remote exposes `./Button` component via `ModuleFederationPlugin` in rspack.config.mjs.

The host uses `React.lazy()` to dynamically import `import('remote/Button')`, which loads the remote module at runtime.
