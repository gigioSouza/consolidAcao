import { defineConfig } from 'vite';
import Vue from '@vitejs/plugin-vue';
import Pages from 'vite-plugin-pages';
import Layouts from 'vite-plugin-vue-layouts';
import WindiCSS from 'vite-plugin-windicss';
import Components from 'unplugin-vue-components/vite';
import Icons from 'unplugin-icons/vite';
import IconsResolver from 'unplugin-icons/resolver';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    Vue(),
    Pages({
      extensions: ['vue'],
      exclude: [
        '**/components/*.*',
        '**/store/*.*',
        '**/composables/*.*'
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
      resolvers: [
        IconsResolver({
          componentPrefix: 'icon'
        })
      ],
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
