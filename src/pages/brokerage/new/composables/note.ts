import { computed, reactive, Ref, UnwrapRef } from 'vue';
import { BrokerageNote, newBrokerageNote, OrderType } from '../../../../tauri/brokerage';
import { helpers, minLength, numeric, required } from '@vuelidate/validators';
import useVuelidate from '@vuelidate/core';
import { DateTime } from 'luxon';
import { orderRules, useOrder } from './orders';
import { useRouter } from 'vue-router';
import { useGlobalLoader } from '../../../../store/global-loader';
import { cloneDeep } from 'lodash-es';

const DATE_FORMAT = 'yyyy-LL-dd';

export function useNote(symbolRef: Ref) {
  const loader = useGlobalLoader();

  const note: UnwrapRef<BrokerageNote> = reactive({
    broker: null,
    total_settlement_fee: 0,
    total_emolument_fee: 0,
    total_broker_fee: 0,
    total_iss_tax: 0,
    trading_date: null,
    orders: []
  });
  note.trading_date = DateTime.now().toFormat(DATE_FORMAT);

  const $note = useVuelidate(noteRules(), note);

  const router = useRouter();

  function cancelNote() {
    router.push({
      name: 'brokerage-list'
    });
  }

  async function saveNote() {
    try {
      loader.show();
      const brokerageNote = cloneDeep(note);
      brokerageNote.trading_date = DateTime.fromFormat(brokerageNote.trading_date!, DATE_FORMAT).toISO();
      await newBrokerageNote(brokerageNote);
      cancelNote();
    } catch (error) {
      // TODO tratar erro
      console.error('error', error);
    } finally {
      loader.hide();
    }
  }

  const totalPurchased = computed(() => {
    return note.orders.reduce((sum, order) => {
      if (OrderType.BUY === order.order_type) {
        return sum + order.order_value;
      }
      return sum;
    }, 0);
  });

  const totalSold = computed(() => {
    return note.orders.reduce((sum, order) => {
      if (OrderType.SELL === order.order_type) {
        return sum + order.order_value;
      }
      return sum;
    }, 0);
  });

  const totalTransacted = computed(() => {
    return totalPurchased.value + totalSold.value;
  });

  return {
    $note,
    cancelNote,
    saveNote,
    totalPurchased,
    totalSold,
    totalTransacted,
    ...useOrder(note, symbolRef)
  }
}

function noteRules() {
  return {
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
      minLength: minLength(1),
      $each: helpers.forEach(orderRules())
    }
  }
}
