import { App } from 'vue';
import { DateTime } from 'luxon';
import { OrderType } from '../tauri/brokerage';
import { vMoneyConfig } from './money';
import { format, unformat } from 'v-money3';

export default {
  install
}

function install(app: App) {
  app.config.globalProperties.$filters = {
    dateFromISO,
    dateFromDatePicker,
    orderType,
    toMoney,
    fromMoney
  }
}

function dateFromISO(date: string | Date, format = 'dd/LL/yyyy'): string {
  const ldate = typeof date === 'string' ? DateTime.fromISO(date) : DateTime.fromJSDate(date);
  return ldate.toFormat(format);
}

function dateFromDatePicker(date: string, format = 'dd/LL/yyyy'): string {
  return date != null ? DateTime.fromFormat(date, 'yyyy-LL-dd').toFormat(format) : '--';
}

function orderType(order_type: OrderType): string {
  return order_type === OrderType.BUY ? 'Compra' : 'Venda';
}

function toMoney(value: string|number = null): string {
  return format(value, vMoneyConfig)
}

function fromMoney(value: string): string|number {
  return unformat(value, vMoneyConfig)
}
