<route>
path: /brokers
name: brokers
meta:
  title: Corretoras
  layout: MenuLayout
</route>

<script lang="ts" setup>
import { useGlobalLoader } from '../store/global-loader';
import { onMounted, Ref, ref } from 'vue';
import { Broker, getBrokerList, newBroker, updateBroker } from '../tauri/brokers';

const loader = useGlobalLoader();

const brokerNameInput: Ref<any> = ref(null);
onMounted(focusInput);
function focusInput() {
  brokerNameInput.value.focus();
}

const brokerName: Ref<string|null> = ref('');
const editingBroker: Ref<Broker|null> = ref(null);

function selectToEdit(broker: Broker) {
  if (editingBroker.value === broker) {
    editingBroker.value = null;
    brokerName.value = null;
  } else {
    editingBroker.value = broker;
    brokerName.value = editingBroker.value.name;
    focusInput();
  }
}
function cancelForm() {
  brokerName.value = null;
  editingBroker.value = null;
}
async function saveBroker() {
  try {
    if (editingBroker.value == null) {
      const broker = await newBroker(brokerName.value as string);
      brokers.value.push(broker);
    } else {
      await updateBroker({ id: editingBroker.value.id, name: brokerName.value })
    }
    brokerName.value = null;
    editingBroker.value = null;
    focusInput()
  } catch (error) {
    // TODO tratar erro
    console.error('Erro ao salvar corretora', error);
  }
}

const brokers: Ref<Broker[]> = ref([]);
onMounted(fetchBrokers)
async function fetchBrokers() {
  try {
    loader.show();
    brokers.value = await getBrokerList();
  } catch (error) {
  } finally {
    loader.hide();
  }
}
</script>

<template>
  <div class="wrapper">
    <form @submit.prevent="saveBroker" @reset="cancelForm" class="broker-form">
      <div class="field">
        <label for="name">Nome:</label>
        <input ref="brokerNameInput" id="name" name="name" type="text" class="input" v-model="brokerName" />
      </div>

      <div class="actions">
        <button type="reset" class="cancel">Cancelar</button>
        <button class="save">Salvar</button>
      </div>
    </form>

    <ul class="broker-list">
      <li v-if="brokers.length === 0">Nenhuma corretora cadastrada.</li>
      <li
        v-for="broker in brokers"
        :key="broker.id"
        :class="{
          editing: editingBroker === broker
        }"
        @click="selectToEdit(broker)">
        {{ broker.name }}
      </li>
    </ul>
  </div>
</template>

<style lang="scss" scoped>
.wrapper {
  @apply flex flex-col;

  .broker-form {
    @apply bg-white rounded shadow-md p-4 flex flex-row justify-start items-center mb-4;

    .field {
      @apply mr-6;

      label {
      @apply text-gray-600 font-light mr-2;
      }
      .input {
       @apply text-base border border-gray-600 rounded px-2;
      }
    }

    .actions {
      button {
        @apply outline-none rounded px-3 py-1 mx-1;
      }

      .cancel {
        @apply bg-cool-gray-500 text-white;
      }
      .save {
        @apply bg-emerald-500 text-white;
      }
    }
  }

  .broker-list {
    @apply bg-white shadow rounded p-4;

    li {
      @apply p-2 w-maxs cursor-pointer;

      &.editing {
        @apply bg-light-blue-700 text-white rounded;
      }

      &:hover:not(&.editing) {
        @apply bg-light-blue-200 rounded;
      }

      &:not(&:last-of-type) {
        @apply border-b border-gray-200;
      }
    }
  }
}
</style>
