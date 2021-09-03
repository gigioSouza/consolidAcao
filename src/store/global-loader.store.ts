import { defineStore } from 'pinia';

export const useGlobalLoader = defineStore('global_loader', {
  state: () => {
    return {
      counter: 0
    };
  },
  getters: {
    isLoading: ({ counter }): boolean => {
      return counter > 0;
    }
  },
  actions: {
    show(): void {
      this.counter++;
    },
    hide(): void {
      if (this.counter > 0) {
        this.counter--;
      }
    }
  }
});
