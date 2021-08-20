<route>
name: welcome
meta:
  layout: Empty
</route>

<script setup>
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import {
  ConfigError,
  getConfigDirPath,
  checkConfig,
  createConfigDir,
  createDatabase
} from '~/tauri/config';

const loading = ref(false);
const isNewUser = ref(false);
const dbNotFound = ref(false);
const configDirPath = ref(null);
const router = useRouter();

onMounted(setup)

async function setup() {
  try {
    loading.value = true;
    await checkConfig();
    router.replace({
      name: 'home'
    });
  } catch (error) {
    switch (error) {
      case ConfigError.CONFIG_DIR_NOT_FOUND:
        isNewUser.value = true;
        break;
      case ConfigError.DB_FILE_NOT_FOUND:
        dbNotFound.value = true;
        configDirPath.value = await getConfigDirPath();
        break;
      default: // TODO
        console.error('error', error);
    }
  } finally {
    loading.value = false;
  }
}

async function setupAppConfig() {
  try {
    loading.value = true;
    await createConfigDir();
    await createDatabase();
    router.replace({
      name: 'home'
    })
  } catch (error) {
    console.error('error', error);
  } finally {
    loading.value = false;
  }
}

async function createNewDatabase() {
  try {
    loading.value = true;
    await createDatabase();
    router.replace({
      name: 'home'
    })
  } catch (error) {
    console.error('error', error);
  } finally {
    loading.value = false;
  }
}

</script>

<template>
  <div class="welcome">
    <div class="card">
      <h1 class="banner">
        Bem-vindo ao ConsolidAção
      </h1>
      <i-icomoon-free-spinner5 v-if="loading" class="loader animate-spin" />
      <div v-else class="prompt">
        <template v-if="isNewUser">
          <p>
            Vejo que é um usuário novo, certo? <br/>
            Clique no botão abaixo e inicar suas configurações.
          </p>
          <button type="button" @click="setupAppConfig">Inicar configuração</button>
        </template>
        <template v-if="dbNotFound">
          <p>
            Por algum motivo não consigo encontrar seus dados. <br/>
            Eles deveriam estar localizados no diretório: <br/>
            <code>{{ configDirPath }}</code>
          </p>
          <button type="button" @click="createNewDatabase">Criar novo banco de dados</button>
        </template>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.welcome {
  @apply h-screen w-screen flex flex-col justify-center items-center bg-light-blue-100;

  .card {
    @apply bg-white rounded-xl shadow-lg p-8 flex flex-col justify-center items-center;

    .banner {
      @apply text-light-blue-700 text-3xl font-sans mb-8;
    }

    .loader {
      @apply text-light-blue-700 text-3xl my-8;
    }

    .prompt {
      @apply p-4 rounded-xl border border-gray-200 text-center;

      p {
        @apply text-gray-800 text-left mb-4;

        code {
          @apply text-sm bg-gray-200 px-2 py-1 rounded shadow;
        }
      }

      button {
        @apply bg-light-blue-600 text-white rounded py-1 px-2;
      }
    }
  }
}
</style>
