import { defineStore } from 'pinia';

export const useMenu = defineStore('menu', {
  state() {
    return {
      collapsed: false,
      hasCollapsed: false
    }
  },
  getters: {
    isCollapsed({ collapsed }) {
      return collapsed;
    },
    isExpanded({ hasCollapsed, collapsed }) {
      return hasCollapsed && !collapsed;
    }
  },
  actions: {
    expand() {
      this.collapsed = false;
    },
    collapse() {
      this.collapsed = true;
      this.hasCollapsed = true;
    }
  }
});
