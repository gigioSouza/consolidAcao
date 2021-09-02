import { defineStore } from 'pinia';
import { useConfig } from './config';

export const useMenu = defineStore('menu', {
  state: () => {
    const configStore = useConfig();

    return {
      collapsed: configStore.config.menuCollapsed,
      hasCollapsed: false
    }
  },
  getters: {
    isCollapsed: ({ collapsed }) => {
      return collapsed;
    },
    isExpanded: ({ hasCollapsed, collapsed }) => {
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
