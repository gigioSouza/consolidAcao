import { defineStore } from 'pinia';
import { AppConfig, updateConfigFile } from '../tauri/config-file';

export const useConfig = defineStore('config', {
  state: () => {
    return {
      config: null,
      isNewUser: false
    }
  },
  actions: {
    async updateConfig(config: AppConfig) {
      this.config = config;
      await updateConfigFile(config);
    }
  }
});
