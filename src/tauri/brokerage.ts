import { invoke } from '@tauri-apps/api/tauri';
import { Broker } from './brokers';
import { PageRequest, PageResponse } from '../types/table';

export interface Brokerage {
  id: number;
  broker: Broker;
  total_settlement_fee: number;
  total_emolument_fee: number;
  total_broker_fee: number;
  total_iss_tax: number;
  total_income_tax: number;
  total_cost: number;
  total_transacted: number;
  total_purchased: number;
  total_sold: number;
  trading_date: string;
}

export enum OrderType {
  BUY = 'C',
  SELL = 'V'
}

export interface BrokerageOrder {
  id?: number
  order_type: OrderType
  symbol: string
  amount: number
  order_value: number
  unit_value?: number
  settlement_fee?: number
  emolument_fee?: number
  broker_fee?: number
  iss_tax?: number
  income_tax?: number
  total_cost?: number
}

export interface BrokerageNote {
  id?: number
  broker: null|Broker
  total_settlement_fee: number
  total_emolument_fee: number
  total_broker_fee: number
  total_iss_tax: number
  total_income_tax?: number
  total_cost?: number
  total_transacted?: number
  total_purchased?: number
  total_sold?: number
  trading_date: string
  orders: BrokerageOrder[]
}

export function getBrokerageNotePage(pageRequest: PageRequest, brokerId?: number): Promise<PageResponse<Brokerage>> {
  return invoke('get_brokerage_note_page', {
    pageRequest,
    brokerId
  });
}

export function newBrokerageNote(brokerageNote: BrokerageNote): Promise<void> {
  return invoke('new_brokerage_note', { brokerageNote });
}

export function getBrokerageNote(brokerageId: number): Promise<BrokerageNote> {
  return invoke('get_brokerage_note', { brokerageId });
}

export function updateBrokerageNote(brokerageNote: BrokerageNote): Promise<BrokerageNote> {
  return invoke('update_brokerage_note', { brokerageNote });
}

export function deleteBrokerageNote(brokerageId: number): Promise<void> {
  return invoke('delete_brokerage_note', { brokerageId });
}
