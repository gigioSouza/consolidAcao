import { invoke } from '@tauri-apps/api/tauri';

export class Broker {
  id: number|null = null;
  name: string|null = null;
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
