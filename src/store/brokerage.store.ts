import { defineStore } from 'pinia';
import { useGlobalLoader } from './global-loader.store';
import {
  getBrokerageNote,
  newBrokerageNote,
  updateBrokerageNote,
  deleteBrokerageNote,
  OrderType,
  BrokerageNote,
  BrokerageOrder
} from '../tauri/brokerage';
import { DateTime } from 'luxon';
import { cloneDeep } from 'lodash-es';

const DATE_FORMAT = 'yyyy-LL-dd';

export const useBrokerage = defineStore('brokerage-form', {
  state: (): { brokerageNote: BrokerageNote } => {
    return {
      brokerageNote: {
        broker: null,
        total_settlement_fee: 0,
        total_emolument_fee: 0,
        total_broker_fee: 0,
        total_iss_tax: 0,
        trading_date: DateTime.now().toFormat(DATE_FORMAT),
        orders: []
      }
    }
  },
  getters: {
    totalPurchased({ brokerageNote }): number {
      return brokerageNote.orders.reduce((sum: number, order: BrokerageOrder) => {
        if (OrderType.BUY === order.order_type) {
          return sum + order.order_value;
        }
        return sum;
      }, 0);
    },
    totalSold({ brokerageNote }): number {
      return brokerageNote.orders.reduce((sum: number, order: BrokerageOrder) => {
        if (OrderType.SELL === order.order_type) {
          return sum + order.order_value;
        }
        return sum;
      }, 0);
    },
    totalTransacted(): number {
      return this.totalPurchased + this.totalSold;
    }
  },
  actions: {
    async fetchBrokerageNote(id: number) {
      const loader = useGlobalLoader();
      try {
        loader.show();
        const brokerageNote = await getBrokerageNote(id);
        brokerageNote.trading_date = DateTime.fromISO(brokerageNote.trading_date).toFormat(DATE_FORMAT);
        Object.assign(this.brokerageNote, brokerageNote);
      } catch (err) {
        // TODO tratar erro
        console.error('Error', err);
      } finally {
        loader.hide();
      }
    },
    async newBrokerageNote() {
      const loader = useGlobalLoader();
      try {
        loader.show();
        const brokerageNote = cloneDeep(this.brokerageNote);
        brokerageNote.trading_date = DateTime.fromFormat(brokerageNote.trading_date!, DATE_FORMAT).toISO();
        await newBrokerageNote(brokerageNote);
      } catch (err) {
        // TODO tratar erro
        console.error('Error', err);
      } finally {
        loader.hide();
      }
    },
    async updateBrokerageNote() {
      const loader = useGlobalLoader();
      try {
        loader.show();
        let brokerageNote = cloneDeep(this.brokerageNote);
        brokerageNote.trading_date = DateTime.fromFormat(brokerageNote.trading_date!, DATE_FORMAT).toISO();
        brokerageNote = await updateBrokerageNote(brokerageNote);
        brokerageNote.trading_date = DateTime.fromISO(brokerageNote.trading_date).toFormat(DATE_FORMAT);
        Object.assign(this.brokerageNote, brokerageNote);
      } catch (err) {
        // TODO tratar erro
        console.error('Error', err);
      } finally {
        loader.hide();
      }
    },
    async deleteBrokerageNote() {
      const loader = useGlobalLoader();
      try {
        loader.show();
        await deleteBrokerageNote(this.brokerageNote.id!);
      } catch (err) {
        // TODO tratar erro
        console.error('Error', err);
      } finally {
        loader.hide();
      }
    }
  }
})
