<route>
path: new
name: brokerage-new
meta:
  title: Nova Nota de Corretagem
  layout: EmptyLayout
</route>

<script lang="ts" setup>
import { useBroker } from './composables/broker';
import { useNote } from './composables/note';
import { OrderType } from '../../../tauri/brokerage';
import { Ref, ref } from 'vue';
import { Column } from '../../../types/table';

const symbolRef = ref(null);
const { brokers } = useBroker();
const {
  $note,
  cancelNote,
  saveNote,
  totalPurchased,
  totalSold,
  totalTransacted,
  $order,
  addOrder,
  removeOrder,
  resetOrder
} = useNote(symbolRef);
const orderType = OrderType;

const columns: Ref<Column[]> = ref([
  {
    prop: 'type',
    label: 'Tipo de Ordem',
    class: 'type'
  },
  {
    prop: 'symbol',
    label: 'Papel',
    class: 'symbol'
  },
  {
    prop: 'amount',
    label: 'Quantidade',
    class: 'amount'
  },
  {
    prop: 'orderValue',
    label: 'Valor Total',
    class: 'orderValue'
  },
  {
    prop: '',
    label: '',
    class: 'actions'
  }
]);

const currentPage = ref(0);
function goToPage(newPage: number) {
  currentPage.value = newPage;
}
</script>

<template>
  <HeaderSlot>
    <button type="button" class="button cancel" @click="cancelNote">Cancelar</button>
    <button type="button" class="button primary ml-2" :disabled="$note.$invalid" @click="saveNote">
      Salvar <i-mdi-content-save class="icon"/>
    </button>
  </HeaderSlot>

  <div class="card form-brokerage">
    <div class="field">
      <label for="broker">Corretora</label>
      <select id="broker" v-model="$note.broker.$model" class="select">
        <option :value="undefined" selected disabled>Selecione</option>
        <option v-for="broker in brokers" :key="broker.id" :value="broker">{{ broker.name }}</option>
      </select>
    </div>

    <div class="field">
      <label for="tradingDate">Data Pregão</label>
      <input id="tradingDate" type="date" v-model="$note.tradingDate.$model" class="input"/>
    </div>

    <div class="field">
      <label for="totalSettlementFee">Taxa de Liquidação</label>
      <money
        id="totalSettlementFee"
        type="text"
        v-model.number="$note.totalSettlementFee.$model"
        class="input"
        v-bind="$configs.vMoney"/>
    </div>

    <div class="field">
      <label for="totalEmolumentFee">Emolumentos</label>
      <money
        id="totalEmolumentFee"
        type="text"
        v-model.number="$note.totalEmolumentFee.$model"
        class="input"
        v-bind="$configs.vMoney"/>
    </div>

    <div class="field">
      <label for="totalBrokerFee">Corretagem</label>
      <money
        id="totalBrokerFee"
        type="text"
        v-model.number="$note.totalBrokerFee.$model"
        class="input"
        v-bind="$configs.vMoney"/>
    </div>

    <div class="field">
      <label for="totalIssTax">ISS</label>
      <money
        id="totalIssTax"
        type="text"
        v-model.number="$note.totalIssTax.$model"
        class="input"
        v-bind="$configs.vMoney"/>
    </div>
  </div>

  <div class="card orders">
    <form @submit.prevent="addOrder" @reset.prevent="resetOrder" class="form-order">
      <div class="field">
        <label>Tipo de Ordem</label>
        <div class="radio-group inline">
          <label for="orderTypeBuy" class="radio">
            <input id="orderTypeBuy" type="radio" v-model="$order.type.$model" :value="orderType.BUY"/>
            Compra
          </label>
          <label for="orderTypeSell" class="radio">
            <input id="orderTypeSell" type="radio" v-model="$order.type.$model" :value="orderType.SELL"/>
            Venda
          </label>
        </div>
      </div>

      <div class="field">
        <label for="symbol">Papel</label>
        <input ref="symbolRef" id="symbol" type="text" v-model="$order.symbol.$model" class="input symbol"/>
      </div>

      <div class="field">
        <label for="amount">Quantidade</label>
        <input id="amount" type="text" v-model.number="$order.amount.$model" class="input"/>
      </div>

      <div class="field">
        <label for="orderValue">Valor total</label>
        <money
          id="orderValue"
          type="text"
          v-model.number="$order.orderValue.$model"
          class="input"
          v-bind="$configs.vMoney"/>
      </div>

      <div class="actions">
        <button type="reset" class="button cancel">Cancelar</button>
        <button class="button secondary" :disabled="$order.$invalid">Adicionar</button>
      </div>
    </form>

    <hr/>

    <Table
      class="table"
      :columns="columns"
      :items="$note.orders.$model"
      empty-message="Adicione ordens através do formulário acima.">
      <template #item="{ item, index }">
        <td class="orderType">{{ $filters.orderType(item.type) }}</td>
        <td class="symbol">{{ item.symbol }}</td>
        <td class="amount">
          <input type="text" v-model.number="item.amount" class="input"/>
        </td>
        <td class="orderValue">
          <money type="text" v-model.number="item.orderValue" class="input" v-bind="$configs.vMoney"/>
        </td>
        <td>
          <button type="button" @click="removeOrder(item)">
            <i-mdi-trash-can class="icon"/>
          </button>
        </td>
      </template>
      <template v-if="totalTransacted > 0" #footer>
        <tr>
          <td colspan="4" class="totalTransacted">{{ $filters.toMoney(totalTransacted) }}</td>
          <td></td>
        </tr>
      </template>
    </Table>
  </div>

  <div v-if="totalTransacted > 0" class="card">
    Total Comprado: {{ $filters.toMoney(totalPurchased) }} <br/>
    Total Vendido: {{ $filters.toMoney(totalSold) }}
  </div>
</template>

<style lang="scss" scoped>
.form-brokerage {
  @apply grid grid-cols-3 gap-2;
}

.orders {
  @apply my-4;

  .form-order {
    @apply grid grid-cols-3 gap-2 mb-4;

    .symbol {
      @apply uppercase;
    }

    .actions {
      @apply col-span-2 flex flex-row justify-end items-end;

      .button.cancel {
        @apply mr-2;
      }
    }
  }

  .table {
    td {
      &.amount,
      &.orderValue,
      &.totalTransacted {
        @apply text-right;
      }

      &.totalTransacted {
        @apply px-4;
      }
    }
  }
}
</style>
