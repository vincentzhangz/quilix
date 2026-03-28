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
