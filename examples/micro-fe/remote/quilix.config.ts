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
