<route>
name: config
meta:
  title: Configurações
  layout: MenuLayout
</route>

<script lang="ts" setup>
import { ref, Ref } from 'vue';
import { AppConfig } from '../tauri/config-file';
import { useGlobalLoader } from '../store/global-loader.store';
import { useConfig } from '../store/config.store';
import { cloneDeep } from 'lodash-es';

const loader = useGlobalLoader();
const store = useConfig();
const config: Ref<AppConfig> = ref(cloneDeep(store.config));

async function saveConfig() {
  try {
    loader.show();
    await store.updateConfig(config.value);
  } catch (err) {
    // TODO tratar erro
    console.error('Error', err);
  } finally {
    loader.hide();
  }
}
</script>

<template>
  <div class="card">
    <div class="field">
      <label>Como deseja que o menu esteja ao iniciar o programa?</label>
      <div class="radio-group inline">
        <label for="menuExpanded" class="radio">
          <input id="menuExpanded" type="radio" v-model="config.menuCollapsed" :value="false"/>
          Aberto
        </label>
        <label for="menuCollapsed" class="radio">
          <input id="menuCollapsed" type="radio" v-model="config.menuCollapsed" :value="true"/>
          Fechado
        </label>
      </div>
    </div>
  </div>

  <HeaderSlot>
    <button type="button" class="button primary" @click="saveConfig">
      Salvar <icon-mdi-content-save class="icon"/>
    </button>
  </HeaderSlot>
</template>

<style lang="scss" scoped>
</style>
