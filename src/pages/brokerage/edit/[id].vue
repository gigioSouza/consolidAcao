<route>
name: brokerage-edit
meta:
  title: Editar Nota de Corretagem
  layout: MenuLayout
</route>

<script lang="ts" setup>
import { onMounted, onUnmounted } from 'vue';
import { useBrokerage } from '../../../store/brokerage.store';
import { useRoute, useRouter } from 'vue-router';
import useVuelidate from '@vuelidate/core';

const route = useRoute();
const store = useBrokerage();
onMounted(() => store.fetchBrokerageNote(+route.params.id));
onUnmounted(() => store.$reset());

const v = useVuelidate({ $stopPropagation: true });

const router = useRouter();
async function saveBrokerageNote() {
  await store.updateBrokerageNote();
}
async function deleteBrokerageNote() {
  await store.deleteBrokerageNote();
  router.push({
    name: 'brokerage-list'
  });
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
    <Button
      variant="light"
      @click="router.back()">
      Voltar
    </Button>
    <Button
      variant="danger"
      class="mx-2"
      icon="delete"
      @click="deleteBrokerageNote">
      Excluir
    </Button>
    <Button
      variant="primary"
      icon="save"
      :disabled="v.$invalid"
      @click="saveBrokerageNote">
      Salvar
    </Button>
  </HeaderSlot>
</template>

<style scoped>

</style>
