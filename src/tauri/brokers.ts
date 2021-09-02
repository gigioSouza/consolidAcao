import { invoke } from '@tauri-apps/api/tauri';

export interface Broker {
  id: number
  name: string
}

export function newBroker(brokerName: string): Promise<Broker> {
  return invoke('new_broker', { brokerName });
}

export function getBrokerList(): Promise<Broker[]> {
  return invoke('get_broker_list');
}

export function updateBroker(broker: Broker): Promise<Broker[]> {
  return invoke('update_broker', { broker });
}
