<route>
path: ''
name: brokerage-list
meta:
  title: Corretagem
</route>

<script lang="ts" setup>
import { Column, PageRequest, SortedBy } from '../../types/table';
import { Broker } from '../../tauri/brokers';
import { onMounted, ref, Ref } from 'vue';
import { useGlobalLoader } from '../../store/global-loader';
import { Brokerage, BrokerageNote, getBrokerageNotePage } from '../../tauri/brokerage';

const columns: Ref<Column[]> = ref([
  {
    prop: 'broker.name',
    label: 'Corretora',
    class: 'broker'
  },
  {
    prop: 'tradingDate',
    label: 'Data Pregão',
    sortable: true,
    sortedBy: SortedBy.DESC,
    class: 'tradingDate'
  },
  {
    prop: 'totalPurchased',
    label: 'Total Comprado',
    class: 'totalPurchased'
  },
  {
    prop: 'totalSold',
    label: 'Total Vendido',
    class: 'totalSold'
  },
  {
    prop: 'totalTransacted',
    label: 'Total Transacionado',
    class: 'totalTransacted'
  }
]);

const brokerageNotes: Ref<Brokerage[]> = ref([]);
const page = ref(0);
const size = ref(10);
const sortBy = ref('trading_date');
const direction = ref(SortedBy.DESC);
const totalPages = ref(0);
const totalElements = ref(0);
onMounted(fetchBrokerPage)
const loader = useGlobalLoader();
async function fetchBrokerPage() {
  try {
    loader.show();
    const pageResponse = await getBrokerageNotePage(new PageRequest(page.value, size.value, sortBy.value, direction.value));
    brokerageNotes.value = pageResponse.content;
    totalPages.value = pageResponse.totalPages;
    totalElements.value = pageResponse.totalElements;
  } catch (error) {
    // TODO tratar erro
    console.error('error', error);
  } finally {
    loader.hide();
  }
}

function onGoToPage(newPage: number) {
  page.value = newPage;
  fetchBrokerPage();
}

function onPageSizeChange(newSize: number) {
  console.log(newSize);
  size.value = newSize;
  fetchBrokerPage();
}
</script>

<template>
  <HeaderSlot>
    <router-link :to="{ name: 'brokerage-new' }" class="button primary">Nova</router-link>
  </HeaderSlot>

  <Table
    :columns="columns"
    :items="brokerageNotes"
    pageable
    :current-page="page"
    :page-size="size"
    :total-pages="totalPages"
    :total-elements="totalElements"
    @goToPage="onGoToPage"
    @pageSizeChange="onPageSizeChange"
    empty-message="Nenhuma nota de corretagem cadastrada.">
    <template #item="{ item }">
      <td class="broker">{{ item.broker.name }}</td>
      <td class="tradingDate">{{ $filters.dateFromISO(item.tradingDate) }}</td>
      <td class="totalPurchased">{{ $filters.toMoney(item.totalPurchased) }}</td>
      <td class="totalSold">{{ $filters.toMoney(item.totalSold) }}</td>
      <td class="totalTransacted">{{ $filters.toMoney(item.totalTransacted) }}</td>
    </template>
  </Table>
</template>

<style lang="scss" scoped>
</style>
