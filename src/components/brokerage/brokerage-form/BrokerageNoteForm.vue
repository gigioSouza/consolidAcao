<script
  lang="ts"
  setup>
import { ref, Ref } from 'vue';
import { OrderType } from '../../../tauri/brokerage';
import { useBrokers } from '../../../composables/brokers';
import { Column } from '../../../types/table';
import { useOrder, OrderRules } from './orders.composable';
import { useBrokerage } from '../../../store/brokerage.store';
import useVuelidate from '@vuelidate/core';
import { minLength, numeric, required } from '@vuelidate/validators';

const symbolRef = ref(null);

const store = useBrokerage();
const $brokerageNote = useVuelidate({
  broker: {
    required
  },
  total_settlement_fee: {
    required,
    numeric
  },
  total_emolument_fee: {
    required,
    numeric
  },
  total_broker_fee: {
    required,
    numeric
  },
  total_iss_tax: {
    required,
    numeric
  },
  trading_date: {
    required
  },
  orders: {
    required,
    minLength: minLength(1)
  }
}, store.brokerageNote, { $registerAs: 'brokerageNoteForm' });

const { brokers } = useBrokers();

const {
  $order,
  addOrder,
  removeOrder,
  resetOrder
} = useOrder(store.brokerageNote, symbolRef);

const v = useVuelidate();

const columns: Ref<Column[]> = ref([
  {
    prop: 'order_type',
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
    prop: 'order_value',
    label: 'Valor Total',
    class: 'orderValue'
  },
  {
    prop: '',
    label: '',
    class: 'actions'
  }
]);
</script>

<template>
  <div class="card form-brokerage">
    <div class="field">
      <label for="broker">Corretora</label>
      <select
        id="broker"
        v-model="$brokerageNote.broker.$model"
        class="select">
        <option
          :value="null"
          selected
          disabled>Selecione
        </option>
        <option
          v-for="broker in brokers"
          :key="broker.id"
          :value="broker">{{ broker.name }}
        </option>
      </select>
    </div>

    <div class="field">
      <label for="tradingDate">Data Pregão</label>
      <input
        id="tradingDate"
        type="date"
        v-model="$brokerageNote.trading_date.$model"
        class="input" />
    </div>

    <div class="field">
      <label for="totalSettlementFee">Taxa de Liquidação</label>
      <money
        id="totalSettlementFee"
        type="text"
        v-model.number="$brokerageNote.total_settlement_fee.$model"
        class="input"
        v-bind="$configs.vMoney" />
    </div>

    <div class="field">
      <label for="totalEmolumentFee">Emolumentos</label>
      <money
        id="totalEmolumentFee"
        type="text"
        v-model.number="$brokerageNote.total_emolument_fee.$model"
        class="input"
        v-bind="$configs.vMoney" />
    </div>

    <div class="field">
      <label for="totalBrokerFee">Corretagem</label>
      <money
        id="totalBrokerFee"
        type="text"
        v-model.number="$brokerageNote.total_broker_fee.$model"
        class="input"
        v-bind="$configs.vMoney" />
    </div>

    <div class="field">
      <label for="totalIssTax">ISS</label>
      <money
        id="totalIssTax"
        type="text"
        v-model.number="$brokerageNote.total_iss_tax.$model"
        class="input"
        v-bind="$configs.vMoney" />
    </div>
  </div>

  <div class="card orders">
    <form
      @submit.prevent="addOrder"
      @reset.prevent="resetOrder"
      class="form-order">
      <div class="field">
        <label>Tipo de Ordem</label>
        <div class="radio-group inline">
          <label
            for="orderTypeBuy"
            class="radio">
            <input
              id="orderTypeBuy"
              type="radio"
              v-model="$order.order_type.$model"
              :value="OrderType.BUY" />
            Compra
          </label>
          <label
            for="orderTypeSell"
            class="radio">
            <input
              id="orderTypeSell"
              type="radio"
              v-model="$order.order_type.$model"
              :value="OrderType.SELL" />
            Venda
          </label>
        </div>
      </div>

      <div class="field">
        <label for="symbol">Papel</label>
        <input
          ref="symbolRef"
          id="symbol"
          type="text"
          v-model="$order.symbol.$model"
          class="input symbol" />
      </div>

      <div class="field">
        <label for="amount">Quantidade</label>
        <input
          id="amount"
          type="text"
          v-model.number="$order.amount.$model"
          class="input" />
      </div>

      <div class="field">
        <label for="orderValue">Valor total</label>
        <money
          id="orderValue"
          type="text"
          v-model.number="$order.order_value.$model"
          class="input"
          v-bind="$configs.vMoney" />
      </div>

      <div class="actions">
        <Button
          reset
          variant="light"
          class="mr-2">
          Cancelar
        </Button>
        <Button
          submit
          variant="secondary"
          :disabled="$order.$invalid">
          Adicionar
        </Button>
      </div>
    </form>

    <hr />

    <Table
      class="table"
      :columns="columns"
      :items="store.brokerageNote.orders"
      empty-message="Adicione ordens através do formulário acima.">
      <template #item="{ item, index }">
        <ValidateEach :state="item" :rules="OrderRules" :key="`order_validation_${index}`">
          <template #default="{ v }">
            <tr>
              <td class="orderType">
                <select v-model="v.order_type.$model" class="select">
                  <option :value="OrderType.BUY">Compra</option>
                  <option :value="OrderType.SELL">Venda</option>
                </select>
              </td>
              <td class="symbol">
                <input type="text" v-model="v.symbol.$model" class="input symbol"/>
              </td>
              <td class="amount">
                <input type="text" v-model.number="v.amount.$model" class="input"/>
              </td>
              <td class="orderValue">
                <money type="text" v-model.number="v.order_value.$model" class="input" v-bind="$configs.vMoney"/>
              </td>
              <td>
                <Button
                  variant="danger"
                  @click="removeOrder(item)"
                  icon="delete"/>
              </td>
            </tr>
          </template>
        </ValidateEach>
      </template>
      <template
        v-if="store.totalTransacted > 0"
        #footer>
        <tr>
          <td
            colspan="4"
            class="totalTransacted">{{ $filters.toMoney(store.totalTransacted) }}
          </td>
          <td></td>
        </tr>
      </template>
    </Table>
  </div>
</template>

<style
  lang="scss"
  scoped>
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
