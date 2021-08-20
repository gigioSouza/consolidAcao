import { defineConfig } from 'vite';
import Vue from '@vitejs/plugin-vue';
import Pages from 'vite-plugin-pages';
import Layouts from 'vite-plugin-vue-layouts';
import WindiCSS from 'vite-plugin-windicss';
import Components from 'vite-plugin-components';
import Icons, { ViteIconsResolver } from 'vite-plugin-icons';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    Vue(),
    Pages({
      extensions: ['vue'],
      exclude: [
        '**/components/*.*',
        '**/store/*.*'
      ],
      routeBlockLang: 'yaml'
    }),
    Layouts({
      exclude: [
        '**/components/*.*',
        '**/store/*.*'
      ]
    }),
    WindiCSS({
      transformCSS: 'pre'
    }),
    Components({
      customComponentResolvers: ViteIconsResolver(),
      globalComponentsDeclaration: true
    }),
    Icons()
  ],
  resolve: {
    alias: [
      { find: '~', replacement: '/src' }
    ]
  },
  server: {
    port: 8080
  }
});
