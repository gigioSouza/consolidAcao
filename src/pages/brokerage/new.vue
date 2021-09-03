<route>
name: brokerage-new
meta:
  title: Nova Nota de Corretagem
  layout: EmptyLayout
</route>

<script
  lang="ts"
  setup>
import { onUnmounted } from 'vue';
import { useBrokerage } from '../../store/brokerage.store';
import { useRouter } from 'vue-router';
import useVuelidate from '@vuelidate/core';

const store = useBrokerage();
onUnmounted(() => store.$reset());

const v = useVuelidate({ $stopPropagation: true });

const router = useRouter();

function cancel() {
  router.push({
    name: 'brokerage-list'
  });
}

async function saveBrokerageNote() {
  await store.newBrokerageNote();
  cancel();
}
</script>

<template>
  <BrokerageNoteForm />

  <div
    v-if="store.totalTransacted > 0"
    class="card">
    Total Comprado: {{ $filters.toMoney(store.totalPurchased) }} <br />
    Total Vendido: {{ $filters.toMoney(store.totalSold) }}
  </div>

  <HeaderSlot>
    <button
      type="button"
      class="button cancel"
      @click="cancel">Cancelar
    </button>
    <button
      type="button"
      class="button primary ml-2"
      :disabled="v.$invalid"
      @click="saveBrokerageNote">
      Salvar
      <icon-mdi-content-save class="icon" />
    </button>
  </HeaderSlot>
</template>
