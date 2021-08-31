import { invoke } from '@tauri-apps/api/tauri';
import { Broker } from './brokers';
import { PageRequest, PageResponse } from '../types/table';

export interface Brokerage {
  id: number;
  broker: Broker;
  totalSettlementFee: number;
  totalEmolumentFee: number;
  totalBrokerFee: number;
  totalIssTax: number;
  totalIncomeTax: number;
  totalCost: number;
  totalTransacted: number;
  totalPurchased: number;
  totalSold: number;
  tradingDate: string;
}

export enum OrderType {
  BUY = 'C',
  SELL = 'V'
}

export class BrokerageOrder {
  id?: number;
  type = OrderType.BUY;
  symbol: string = '';
  amount = 0;
  orderValue = 0;
  unitValue?: number;
  settlementFee?: number;
  emolumentFee?: number;
  brokerFee?: number;
  issTax?: number;
  incomeTax?: number;
  totalCost?: number;
}

export class BrokerageNote {
  id?: number;
  broker?: Broker;
  totalSettlementFee = 0;
  totalEmolumentFee = 0;
  totalBrokerFee = 0;
  totalIssTax = 0;
  totalIncomeTax = 0;
  totalCost?: number;
  totalTransacted?: number;
  totalPurchased?: number;
  totalSold?: number;
  tradingDate?: string;
  orders: BrokerageOrder[] = []
}

export function getBrokerageNotePage(pageRequest: PageRequest, brokerId?: number): Promise<PageResponse<Brokerage>> {
  console.log(pageRequest);
  return invoke('get_brokerage_note_page', {
    pageRequest,
    brokerId
  })
}

export function newBrokerageNote(brokerageNote: BrokerageNote): Promise<void> {
  return invoke('new_brokerage_note', { brokerageNote });
}
