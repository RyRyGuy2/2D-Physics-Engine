import { defineConfig } from 'vite';

export default defineConfig({
  server: {
    fs: {
      // wasm-pack outputs a sibling package outside the frontend root.
      allow: ['.', '../physics/pkg'],
    },
  },
});
