import { onMounted, ref, Ref } from 'vue';
import { Broker, getBrokerList } from '../../../../tauri/brokers';
import { useGlobalLoader } from '../../../../store/global-loader';

const loader = useGlobalLoader();

export function useBroker() {
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
  }
}
