import { computed, reactive, Ref } from 'vue';
import { BrokerageNote, newBrokerage, newBrokerageNote, OrderType } from '../../../../tauri/brokerage';
import { minLength, numeric, required } from '@vuelidate/validators';
import useVuelidate from '@vuelidate/core';
import { DateTime } from 'luxon';
import { useOrder } from './orders';
import { useRouter } from 'vue-router';
import { useGlobalLoader } from '../../../../store/global-loader';
import { cloneDeep } from 'lodash-es';

const DATE_FORMAT = 'yyyy-LL-dd';

export function useNote(symbolRef: Ref) {
  const note = reactive(new BrokerageNote());
  note.tradingDate = DateTime.now().toFormat(DATE_FORMAT);

  const loader = useGlobalLoader();

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
      brokerageNote.tradingDate = DateTime.fromFormat(brokerageNote.tradingDate!, DATE_FORMAT).toISO();
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
      if (OrderType.BUY === order.type) {
        return sum + order.orderValue;
      }
      return sum;
    }, 0);
  });

  const totalSold = computed(() => {
    return note.orders.reduce((sum, order) => {
      if (OrderType.SELL === order.type) {
        return sum + order.orderValue;
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
    totalSettlementFee: {
      required,
      numeric
    },
    totalEmolumentFee: {
      required,
      numeric
    },
    totalBrokerFee: {
      required,
      numeric
    },
    totalIssTax: {
      required,
      numeric
    },
    tradingDate: {
      required
    },
    orders: {
      required,
      minLength: minLength(1)
    }
  }
}
