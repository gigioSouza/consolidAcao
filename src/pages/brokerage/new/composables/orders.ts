import { BrokerageNote, BrokerageOrder } from '../../../../tauri/brokerage';
import { Ref, ref } from 'vue';
import useVuelidate from '@vuelidate/core';
import { numeric, required } from '@vuelidate/validators';

export function useOrder(note: BrokerageNote, symbolRef: Ref) {
  const order = ref(new BrokerageOrder());
  const $order = useVuelidate(orderRules(), order);

  function resetOrder() {
    order.value = new BrokerageOrder();
  }

  function addOrder() {
    order.value.symbol = order.value.symbol.toUpperCase();
    note.orders.push(order.value);
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
    removeOrder
  }
}

function orderRules() {
  return {
    type: {
      required
    },
    'symbol': {
      required
    },
    amount: {
      required,
      numeric
    },
    orderValue: {
      required,
      numeric
    }
  }
}
