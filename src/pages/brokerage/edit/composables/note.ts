import { BrokerageNote, getBrokerageNote } from '../../../../tauri/brokerage';
import { onMounted, ref, Ref } from 'vue';

export function useNote(id: number) {

  const note: Ref<BrokerageNote> = ref(null);

  onMounted(async () => {
    console.log('onMounted')
    note.value = await getBrokerageNote(id);
    console.log(note.value);
  });

  return {
    note
  }
}
