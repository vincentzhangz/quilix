import { defineConfig } from '@quilix/core';
import { moduleFederation } from '@quilix/core';

export default defineConfig({
  plugins: [
    moduleFederation({
      name: 'host',
      remotes: {},
      shared: ['react', 'react-dom'],
    }),
  ],
});
