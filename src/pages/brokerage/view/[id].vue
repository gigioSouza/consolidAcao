<route>
name: brokerage-view
meta:
  title: Visualizar
  layout: MenuLayout
</route>

<script lang="ts" setup>
import { useRoute, useRouter } from 'vue-router';
import { computed, onMounted, onUnmounted, ref } from 'vue';
import { useBrokerage } from '../../../store/brokerage.store';
import { OrderType } from '../../../tauri/brokerage';
import viewDetailsIcon from '~icons/mdi/chevron-down';
import hideDetailsIcon from '~icons/mdi/chevron-up';

const orderType = OrderType;

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

const orderFilter = ref([OrderType.BUY, OrderType.SELL]);
const purchaseOrders = computed(() => brokerageNote.orders.filter(order => order.order_type === OrderType.BUY));
const sellOrders = computed(() => brokerageNote.orders.filter(order => order.order_type === OrderType.SELL));
const orders = computed(() => {
  if (orderFilter.value.length === 0) {
    return [];
  }
  if (orderFilter.value.length === 1) {
    return orderFilter.value[0] === OrderType.BUY ? purchaseOrders.value : sellOrders.value;
  }
  return brokerageNote.orders;
});

const columns = [
  {
    prop: 'symbol',
    label: 'Papel',
    class: 'text-center'
  },
  {
    prop: 'amount',
    label: 'Quantidade',
    class: 'text-center'
  },
  {
    prop: 'order_value',
    label: 'Valor da Ordem',
    class: 'text-right'
  },
  {
    prop: 'total_cost',
    label: 'Custo da Ordem',
    class: 'text-right'
  },
  {
    prop: '',
    label: '',
    class: 'w-8'
  }
];
</script>

<template>
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
    <div class="orders-header">
      <h2>Total ({{ brokerageNote?.orders?.length }})</h2>

      <div class="orders-filter">
        <div class="field inline">
          <label for="purchaseCheckbox">Compra ({{ purchaseOrders.length }})</label>
          <input
            id="purchaseCheckbox"
            type="checkbox"
            class="checkbox"
            v-model="orderFilter"
            :value="orderType.BUY"
            :disabled="purchaseOrders.length === 0"/>
        </div>
        <div class="field inline">
          <label for="sellCheckbox">Venda ({{ sellOrders.length }})</label>
          <input
            id="sellCheckbox"
            type="checkbox"
            class="checkbox"
            v-model="orderFilter"
            :value="orderType.SELL"
            :disabled="sellOrders.length === 0"/>
        </div>
      </div>
    </div>
    <Table
      :columns="columns"
      :items="orders"
      class="table"
      empty-message="Não há dados para o filtro selecionado."
      expandable>
      <template #item="{ item }">
        <tr class="row" :class="item.order_type == orderType.BUY ? 'buy' : 'sell'">
          <td class="text-center">{{ item.symbol }}</td>
          <td class="text-center">{{ item.amount }}</td>
          <td class="text-right">{{ $filters.toMoney(item.order_value) }}</td>
          <td class="text-right">{{ $filters.toMoney(item.total_cost) }}</td>
          <td class="text-center w-8">
            <Button
              class="text-lg"
              variant="empty"
              :icon="item.expanded ? hideDetailsIcon : viewDetailsIcon"
              @click="item.expanded = !item.expanded"
            />
          </td>
        </tr>
      </template>
      <template #expandable="{ item }">
        <div class="order-detail" :class="item.order_type == orderType.BUY ? 'buy' : 'sell'">
          <div class="field">
            <label>Valor da Unidade</label>
            <span class="input v-money3">{{ $filters.toMoney(item.unit_value) }}</span>
          </div>
          <div class="field">
            <label>Taxa de Liquidação</label>
            <span class="input v-money3">{{ $filters.toMoney(item.settlement_fee) }}</span>
          </div>
          <div class="field">
            <label>Emolumentos</label>
            <span class="input v-money3">{{ $filters.toMoney(item.emolument_fee) }}</span>
          </div>
          <div class="field">
            <label>Corretagem</label>
            <span class="input v-money3">{{ $filters.toMoney(item.broker_fee) }}</span>
          </div>
          <div class="field">
            <label>ISS</label>
            <span class="input v-money3">{{ $filters.toMoney(item.iss_tax) }}</span>
          </div>
          <div class="field">
            <label>IRRF</label>
            <span class="input v-money3">{{ $filters.toMoney(item.income_tax) }}</span>
          </div>
        </div>
      </template>
    </Table>
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
  .orders-header {
    @apply flex flex-row justify-between;

    .orders-filter {
      @apply flex flex-row gap-4;
    }
  }

  .subtitle {
    @apply text-xl text-white bg-light-blue-600 shadow mb-2 p-2 rounded;
  }

  .table {
    .row {
      @apply !border-white;

      &.buy {
        @apply bg-red-600 bg-opacity-20;
      }
      &.sell {
        @apply bg-lime-600 bg-opacity-20;
      }
    }
    .order-detail {
      @apply grid grid-cols-3 gap-2 px-6 py-4;

      &.buy {
        @apply bg-red-600 bg-opacity-10;
      }
      &.sell {
        @apply bg-lime-600 bg-opacity-10;
      }
    }
  }
}
</style>
