import { onMounted, ref, Ref } from 'vue';
import { Broker, getBrokerList } from '../tauri/brokers';
import { useGlobalLoader } from '../store/global-loader.store';

const loader = useGlobalLoader();

export function useBrokers() {
  const brokers: Ref<Broker[]> = ref([]);

  onMounted(fetchBrokerList);

  async function fetchBrokerList() {
    try {
      loader.show();
      brokers.value = await getBrokerList();
    } finally {
      loader.hide();
    }
  }

  return {
    brokers
  };
}
