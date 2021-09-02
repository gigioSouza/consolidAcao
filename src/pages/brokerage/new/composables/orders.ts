import { BrokerageNote, BrokerageOrder, OrderType } from '../../../../tauri/brokerage';
import { Ref, ref, UnwrapRef } from 'vue';
import useVuelidate from '@vuelidate/core';
import { numeric, required } from '@vuelidate/validators';
import { cloneDeep } from 'lodash-es';

export function useOrder(note: UnwrapRef<BrokerageNote>, symbolRef: Ref) {
  const order: Ref<BrokerageOrder> = ref({
    order_type: OrderType.BUY,
    symbol: '',
    amount: 0,
    order_value: 0
  });
  const $order = useVuelidate(orderRules(), order);

  function resetOrder() {
    order.value = {
      order_type: OrderType.BUY,
      symbol: '',
      amount: 0,
      order_value: 0
    };
  }

  function addOrder() {
    order.value.symbol = order.value.symbol.toUpperCase();
    note.orders.push(cloneDeep(order.value));
    resetOrder();
    (symbolRef.value! as HTMLElement).focus();
  }

  function removeOrder(order: BrokerageOrder) {
    const index = note.orders.indexOf(order);
    note.orders.splice(index, 1);
  }

  return {
    $order,
    resetOrder,
    addOrder,
    removeOrder,
    orderRules
  }
}

export function orderRules() {
  return {
    order_type: {
      required
    },
    'symbol': {
      required
    },
    amount: {
      required,
      numeric
    },
    order_value: {
      required,
      numeric
    }
  }
}
