import { createRouter, createWebHistory } from 'vue-router';
import { setupLayouts } from 'virtual:generated-layouts'
import routes from 'virtual:generated-pages';

export default createRouter({
  history: createWebHistory(),
  routes: setupLayouts(routes)
});
