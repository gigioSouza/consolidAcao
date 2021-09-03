<route>
path: ''
name: brokerage-list
meta:
  title: Corretagem
</route>

<script lang="ts" setup>
import { Column, SortedBy } from '../../types/table';
import { onMounted, ref, Ref } from 'vue';
import { useGlobalLoader } from '../../store/global-loader.store';
import { Brokerage, getBrokerageNotePage } from '../../tauri/brokerage';
import { useBrokers } from '../../composables/brokers';
import { usePageable } from '../../composables/pageable';
import { useRouter } from 'vue-router';

const router = useRouter();
const loader = useGlobalLoader();
const broker: Ref<number> = ref(null);
const brokerageNotes: Ref<Brokerage[]> = ref([]);

const { brokers } = useBrokers();

const {
  page,
  size,
  sortBy,
  direction,
  totalPages,
  totalElements,
  onGoToPage,
  onSortBy,
  onPageSizeChange
} = usePageable(fetchBrokerageNotePage);

onMounted(fetchBrokerageNotePage)
async function fetchBrokerageNotePage() {
  try {
    loader.show();

    const { content, total_pages, total_elements } = await getBrokerageNotePage(
      {
        page: page.value,
        size: size.value,
        sort_by: sortBy.value,
        direction: direction.value
      },
      broker.value
    );

    brokerageNotes.value = content;
    totalPages.value = total_pages;
    totalElements.value = total_elements;
  } catch (error) {
    // TODO tratar erro
    console.error('error', error);
  } finally {
    loader.hide();
  }
}

function onBrokerChange() {
  page.value = 0;
  fetchBrokerageNotePage();
}

function onItemClick({ item }) {
  router.push({
    name: 'brokerage-view',
    params: {
      id: item.id
    }
  })
}

const columns: Ref<Column[]> = ref([
  {
    prop: 'broker.name',
    label: 'Corretora',
    class: 'broker'
  },
  {
    prop: 'trading_date',
    label: 'Data Pregão',
    sortable: true,
    sortedBy: SortedBy.DESC,
    class: 'tradingDate'
  },
  {
    prop: 'total_purchased',
    label: 'Total Comprado',
    class: 'totalPurchased'
  },
  {
    prop: 'total_sold',
    label: 'Total Vendido',
    class: 'totalSold'
  },
  {
    prop: 'total_transacted',
    label: 'Total Transacionado',
    class: 'totalTransacted'
  }
]);
</script>

<template>
  <HeaderSlot>
    <router-link :to="{ name: 'brokerage-new' }" class="button primary">Nova <icon-mdi-plus class="icon"/></router-link>
  </HeaderSlot>

  <div class="card">
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
      @sortBy="onSortBy"
      @itemClick="onItemClick"
      empty-message="Nenhuma nota de corretagem cadastrada.">
      <template #header>
        <div class="field inline brokerSelector">
          <label for="broker">Filtrar por corretora</label>
          <select id="broker" v-model="broker" class="select" @change="onBrokerChange">
            <option :value="null" selected>Todas</option>
            <option
              v-for="broker in brokers"
              :key="`broker_${broker.id}`"
              :value="broker.id"
              selected>
              {{ broker.name }}
            </option>
          </select>
        </div>
      </template>
      <template #item="{ item }">
        <td class="broker">{{ item.broker.name }}</td>
        <td class="tradingDate text-center">{{ $filters.dateFromISO(item.trading_date) }}</td>
        <td class="totalPurchased text-right">{{ $filters.toMoney(item.total_purchased) }}</td>
        <td class="totalSold text-right">{{ $filters.toMoney(item.total_sold) }}</td>
        <td class="totalTransacted text-right">{{ $filters.toMoney(item.total_transacted) }}</td>
      </template>
    </Table>
  </div>
</template>

<style lang="scss" scoped>
.brokerSelector {
  .select {
    @apply w-36 mb-1;
  }
}
</style>
