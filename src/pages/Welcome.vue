<route>
name: welcome
path: /
meta:
  layout: Empty
</route>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import {
  checkConfig,
  ConfigError,
  createDatabase,
  openDataDirSelectionDialog,
  createConfigFile
} from '../tauri/config';

enum WelcomeState {
  NEW_USER,
  CONFIRM_DATA_DIR,
  DB_FILE_NOT_FOUND
}

const loading = ref(true);
const state = ref(false);
const dataDir = ref(null);
const router = useRouter();

onMounted(setupConfig);

async function setupConfig() {
  try {
    loading.value = true;
    await checkConfig();
    router.replace({
      name: 'home'
    });
  } catch (error) {
    switch (error) {
      case ConfigError.NEW_USER:
        state.value = WelcomeState.NEW_USER;
        break;
      default:
        console.error('Erro ao verificar configurações', error);
    }
  } finally {
    loading.value = false;
  }
}

async function chooseDataDir() {
  try {
    dataDir.value = `${await openDataDirSelectionDialog()}/consolidAcao`;
    state.value = WelcomeState.CONFIRM_DATA_DIR;
  } catch (error) {
    // TODO tratar erro
    console.error('Erro ao abrir dialogo de seleção de diretório', error);
  }
}

async function confirmDataDir() {
  try {
    loading.value = true;
    await createConfigFile(dataDir.value);
    await createDatabase();
    router.replace({
      name: 'home'
    });
  } catch (error) {
    // TODO tratar erro
    console.error('Erro ao criar arquivo de configuração e criar banco de dados', error);
  } finally {
    loading.value = false;
  }
}

async function createNewDatabase() {
  try {
    loading.value = true;
    await createDatabase();
  } catch (error) {
    // TODO tratar erro
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
      <i-eos-icons-loading v-if="loading" class="loader animate-spin" />
      <div v-else class="prompt">
        <template v-if="state === WelcomeState.NEW_USER">
          <p>
            Clique no botão abaixo e selecione o diretório onde você deseja que este aplicativo armazene seus dados.
          </p>
          <blockquote>
            Todos os dados que você inserir nesta aplicação ficaram armazenados na pasta <code>/consolidAcao</code> dentro do diretório escolhido por você.
          </blockquote>
          <button
            type="button"
            class="choose-data-dir"
            @click="chooseDataDir">
            Selecionar diretório
            <i-mdi-folder-search class="icon"/>
          </button>
        </template>

        <template v-else-if="state === WelcomeState.CONFIRM_DATA_DIR">
          <p>Confirma o caminho de diretório escolhido?</p>
          <code>{{ dataDir }}</code>
          <div class="actions">
            <button
              type="button"
              class="choose-data-dir"
              @click="chooseDataDir">
              Selecionar diretório
              <i-mdi-folder-search class="icon"/>
            </button>

            <button
              type="button"
              class="confirm-data-dir"
              @click="confirmDataDir">
              Confirmar
              <i-mdi-check class="icon"/>
            </button>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.welcome {
  @apply h-screen w-screen flex flex-col justify-center items-center bg-light-blue-100;

  .card {
    @apply max-w-4/6 bg-white rounded-xl shadow-lg p-8 flex flex-col justify-center items-center;

    .banner {
      @apply text-light-blue-700 text-3xl font-sans mb-8;
    }

    .loader {
      @apply text-light-blue-700 text-3xl my-8;
    }

    .prompt {
      @apply p-4 rounded-xl border border-gray-200 flex flex-col items-center;

      p {
        @apply text-gray-800;
      }

      blockquote {
        @apply bg-light-blue-100 p-2 my-4 text-sm font-light rounded shadow-sm border-2 border-light-blue-200;
      }

      code {
        @apply bg-gray-100 border border-gray-400 font-normal px-1 py-0.5;
      }

      button {
        @apply rounded py-2 px-4 flex flex-row items-center;

        .icon {
          @apply ml-2;
        }
      }

      .choose-data-dir {
        @apply bg-light-blue-600 text-white;
      }

      .confirm-data-dir {
        @apply bg-emerald-500 text-white;
      }

      .actions {
        @apply flex flex-row mt-4;

        button {
          @apply mx-1;
        }
      }
    }
  }
}

@screen lg {
  .welcome {
    .card {
      @apply w-1/3;
    }
  }
}
</style>
