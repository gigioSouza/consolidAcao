<route>
name: brokerage-view
meta:
  title: Visualizar
  layout: MenuLayout
</route>

<script lang="ts" setup>
import { useRoute, useRouter } from 'vue-router';
import { onMounted, onUnmounted } from 'vue';
import { useBrokerage } from '../../../store/brokerage.store';
import { OrderType } from '../../../tauri/brokerage';

const route = useRoute();
const store = useBrokerage();
onMounted(() => store.fetchBrokerageNote(+route.params.id));
onUnmounted(() => store.$reset());

const brokerageNote = store.brokerageNote;

const router = useRouter();
function edit() {
  router.push({
    name: 'brokerage-edit',
    params: route.params
  });
}
</script>

<template>
<!--  <pre>{{ brokerageNote }}</pre>-->
  <HeaderSlot>
    <Button
      variant="light"
      class="mr-2"
      @click="router.back()">
      Voltar
    </Button>
    <Button
      variant="primary"
      @click="edit">
      Editar
    </Button>
  </HeaderSlot>

  <div class="card brokerage">
    <div class="field">
      <label>Corretora</label>
      <span class="input">{{ brokerageNote?.broker?.name }}</span>
    </div>
    <div class="field">
      <label>Data Pregão</label>
      <span class="input">{{ $filters.dateFromDatePicker(brokerageNote?.trading_date) }}</span>
    </div>
    <div class="field">
      <label>Taxa de Liquidação</label>
      <span class="input v-money3">{{ $filters.toMoney(brokerageNote?.total_settlement_fee) }}</span>
    </div>
    <div class="field">
      <label>Emolumentos</label>
      <span class="input v-money3">{{ $filters.toMoney(brokerageNote?.total_emolument_fee) }}</span>
    </div>
    <div class="field">
      <label>Corretagem</label>
      <span class="input v-money3">{{ $filters.toMoney(brokerageNote?.total_broker_fee) }}</span>
    </div>
    <div class="field">
      <label>ISS</label>
      <span class="input v-money3">{{ $filters.toMoney(brokerageNote?.total_iss_tax) }}</span>
    </div>
    <div class="field">
      <label>IRRF</label>
      <span class="input v-money3">{{ $filters.toMoney(brokerageNote?.total_income_tax) }}</span>
    </div>
    <div class="field">
      <label>Custo total da Nota</label>
      <span class="input v-money3">{{ $filters.toMoney(brokerageNote?.total_cost) }}</span>
    </div>
  </div>

  <div class="card totals">
    <div class="field">
      <label>Total Comprado</label>
      <span class="input v-money3">{{ $filters.toMoney(brokerageNote?.total_purchased) }}</span>
    </div>
    <div class="field">
      <label>Total Vendido</label>
      <span class="input v-money3">{{ $filters.toMoney(brokerageNote?.total_sold) }}</span>
    </div>
    <div class="field">
      <label>Total Transacionado</label>
      <span class="input v-money3">{{ $filters.toMoney(brokerageNote?.total_transacted) }}</span>
    </div>
  </div>

  <div class="card orders">
    <h2 class="subtitle">Ordens ({{ brokerageNote?.orders?.length }})</h2>
    <div
      v-for="order in brokerageNote.orders"
      :key="`order_${order.id}`"
      class="card order"
      :class="{
        buy: order.order_type === OrderType.BUY,
        sell: order.order_type === OrderType.SELL
      }">
      <div class="field">
        <label>Papel</label>
        <span class="input">{{ order.symbol }}</span>
      </div>
      <div class="field">
        <label>Quantidade</label>
        <span class="input">{{ order.amount }}</span>
      </div>
      <div class="field">
        <label>Valor da Ordem</label>
        <span class="input v-money3">{{ $filters.toMoney(order.order_value) }}</span>
      </div>
      <div class="field">
        <label>Valor da Unidade</label>
        <span class="input v-money3">{{ $filters.toMoney(order.unit_value) }}</span>
      </div>
      <div class="field">
        <label>Taxa de Liquidação</label>
        <span class="input v-money3">{{ $filters.toMoney(order.settlement_fee) }}</span>
      </div>
      <div class="field">
        <label>Emolumentos</label>
        <span class="input v-money3">{{ $filters.toMoney(order.emolument_fee) }}</span>
      </div>
      <div class="field">
        <label>Corretagem</label>
        <span class="input v-money3">{{ $filters.toMoney(order.broker_fee) }}</span>
      </div>
      <div class="field">
        <label>ISS</label>
        <span class="input v-money3">{{ $filters.toMoney(order.iss_tax) }}</span>
      </div>
      <div class="field">
        <label>IRRF</label>
        <span class="input v-money3">{{ $filters.toMoney(order.income_tax) }}</span>
      </div>
      <div class="field">
        <label>Custo da Operação</label>
        <span class="input v-money3">{{ $filters.toMoney(order.total_cost) }}</span>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.brokerage,
.totals {
  @apply grid grid-cols-3 gap-2;
}

.totals {
  @apply my-4;
}

.orders {
  .subtitle {
    @apply text-xl text-white bg-light-blue-600 shadow mb-2 p-2 rounded;
  }

  .order {
    @apply grid grid-cols-3 gap-2;

    &:not(&:last-of-type) {
      @apply mb-4;
    }

    &.buy {
      @apply bg-red-600 bg-opacity-20;
    }
    &.sell {
      @apply bg-lime-600 bg-opacity-20;
    }
  }
}
</style>
