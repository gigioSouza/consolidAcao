import { defineConfig } from 'vite-plugin-windicss';

export default defineConfig({
  preflight: true,
  theme: {
    extend: {
      fontFamily: {
        sans: ['Montserrat']
      }
    }
  }
});
