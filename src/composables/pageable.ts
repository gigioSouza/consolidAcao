import { ref } from 'vue';
import { Column, SortedBy } from '../types/table';

export function usePageable(fetchPage) {
  const page = ref(0);
  const size = ref(10);
  const sortBy = ref('trading_date');
  const direction = ref(SortedBy.DESC);
  const totalPages = ref(0);
  const totalElements = ref(0);

  function onGoToPage(newPage: number) {
    page.value = newPage;
    fetchPage();
  }

  function onSortBy(newSort: Column) {
    newSort.sortedBy = newSort.sortedBy === SortedBy.ASC ? SortedBy.DESC : SortedBy.ASC;
    sortBy.value = newSort.prop;
    direction.value = newSort.sortedBy;
    fetchPage();
  }

  function onPageSizeChange(newSize: number) {
    size.value = newSize;
    page.value = 0;
    fetchPage();
  }

  return {
    page,
    size,
    sortBy,
    direction,
    totalPages,
    totalElements,
    onGoToPage,
    onSortBy,
    onPageSizeChange
  }
}
