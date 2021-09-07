<script lang="ts" setup>
import { computed, reactive, ref, toRefs, unref, useSlots, watch } from 'vue';
import { Column, SortedBy } from '../types/table';
import range from 'lodash-es/range';
import get from 'lodash-es/get';
import gsap from 'gsap';

const props = withDefaults(defineProps<{
  columns: Column[],
  items: any[],
  emptyMessage?: string
  pageable?: boolean,
  currentPage?: number,
  pageSize?: number,
  totalPages?: number,
  totalElements?: number,
  expandable?: boolean
}>(), {
  pageable: false,
  currentPage: 0,
  pageSize: 10,
  totalPages: 0,
  totalElements: 0,
  expandable: false
});
const {
  columns,
  emptyMessage,
  pageable,
  currentPage,
  pageSize,
  totalPages,
  totalElements,
  expandable
} = toRefs(props);

const emit = defineEmits<{
  (e: 'goToPage', page: number): void,
  (e: 'sortBy', column: Column): void,
  (e: 'pageSizeChange', size: number): void,
  (e: 'itemClick', { item: any, column: Column }): void
}>();

const slots = useSlots();

const sortedBy = SortedBy;
const showEmptyMessage = computed(() => {
  return props.items == null || props.items.length === 0
});

function sortBy(column: Column) {
  if (column.sortable) {
    emit('sortBy', column);
  }
}

const showPagination = computed(() => pageable.value && totalPages.value > 1);
const pages = computed(() => {
  if (totalPages.value <= 5) {
    return Array.from(Array(totalPages.value).keys());
  }

  if (currentPage.value <= 2) {
    return Array.from(Array(5).keys());
  }

  if (totalPages.value - currentPage.value <= 2) {
    return range(totalPages.value - 5, totalPages.value);
  }

  return range(currentPage.value - 2, currentPage.value + 3);
});

function goToPage(newPage: number) {
  if (newPage >= 0 && newPage < totalPages.value) {
    emit('goToPage', newPage);
  }
}

const isFirstPage = computed(() => currentPage.value === 0);
const isLastPage = computed(() => currentPage.value === totalPages.value - 1);

const selectedSize = ref(pageSize.value);
watch(pageSize, (newValue) => {
  selectedSize.value = newValue;
});

const fromElement = computed(() => {
  return (currentPage.value * pageSize.value) + 1;
});
const toElement = computed(() => {
  const to = (currentPage.value * pageSize.value) + pageSize.value;
  return totalElements.value > to ? to : totalElements.value;
});

const items = computed(() => {
  if (!Array.isArray(props.items)) return [];
  if (expandable.value === false) return props.items;

  return props.items.map(item => {
    return reactive({
      ...item,
      expanded: false
    });
  })
});

function beforeEnter(el) {
  gsap.set(el, {
    height: 0,
    overflow: 'hidden'
  });
}

function enter(el, done) {
  gsap.to(el, {
    duration: .25,
    height: 'auto',
  });
  gsap.to(el, {
    duration: .3,
    onComplete: done
  });
}

function leave(el, done) {
  gsap.to(el, {
    duration: .1,
  });
  gsap.to(el, {
    duration: .3,
    height: 0,
    overflow: 'hidden',
    onComplete: done
  });
}
</script>

<template>
  <div class="table-container">
    <template v-if="pageable">
      <div class="table-header">
        <slot name="header">
          <div></div>
        </slot>

        <div class="field inline pageSizeSelector">
          <label for="pageSize">Itens por página</label>
          <select
            id="pageSize"
            v-model="selectedSize"
            class="select items-end"
            @change="emit('pageSizeChange', selectedSize)">
            <option :value="10" :selected="false">10</option>
            <option :value="25" :selected="false">25</option>
            <option :value="50" :selected="true">50</option>
            <option :value="100" :selected="false">100</option>
          </select>
        </div>
      </div>
    </template>
    <div class="table-wrapper">
      <table class="table">
        <thead>
        <tr>
          <th
            v-for="(col, i) in columns"
            :key="`col_${col.prop}_${i}`"
            @click="sortBy(col)"
            :class="[
              col.class,
              {
                sortable: col.sortable
              }
            ]">
            <span class="wrapper">
              <span class="text">
                {{ col.label }}
              </span>
              <template v-if="col.sortable === true">
                <icon-mdi-menu-up v-if="col.sortedBy === sortedBy.ASC" class="sort-icon"/>
                <icon-mdi-menu-down v-else-if="col.sortedBy === sortedBy.DESC" class="sort-icon"/>
                <icon-mdi-menu-swap v-else class="sort-icon"/>
              </template>
            </span>
          </th>
        </tr>
        </thead>
        <tbody>
          <tr v-if="showEmptyMessage" class="empty-message">
            <td :colspan="columns.length">{{ emptyMessage }}</td>
          </tr>
          <template v-for="(item, i) in items" :key="`row_${i}`">
            <slot v-if="slots.item != null" name="item" :item="item" :index="i"/>
            <template v-else>
              <tr>
                <td
                  v-for="col in columns"
                  :key="`row_${i}_col_${col.prop}`"
                  :class="col.class">
                  {{ get(item, col.prop, '--') }}
                </td>
              </tr>
            </template>
            <tr v-show="expandable" class="expandable" :class="{ expanded: item.expanded }">
              <td :colspan="columns.length">
                <transition
                  name="expanded"
                  @before-enter="beforeEnter"
                  @enter="enter"
                  @leave="leave"
                >
                  <div v-show="item.expanded">
                    <slot name="expandable" :item="item" :index="i" />
                  </div>
                </transition>
              </td>
            </tr>
          </template>
        </tbody>
        <tfoot v-if="slots.footer != null">
          <slot name="footer"/>
        </tfoot>
      </table>
    </div>
    <div v-if="showPagination" class="pageable">
      <p class="legend">
        Apresentando {{ fromElement }} - {{ toElement }} de {{ totalElements }}
      </p>

      <ul class="pagination">
        <li class="page first">
          <button type="button" :disabled="isFirstPage" @click="goToPage(0)">
            <icon-mdi-page-first/>
          </button>
        </li>
        <li class="page prev">
          <button type="button" :disabled="isFirstPage" @click="goToPage(currentPage - 1)">
            <icon-mdi-chevron-left/>
          </button>
        </li>
        <li v-for="page in pages" :key="`page_${page}`" class="page" :class="{ active: currentPage === page }">
          <button type="button" @click="goToPage(page)">{{ page + 1 }}</button>
        </li>
        <li class="page next">
          <button type="button" :disabled="isLastPage" @click="goToPage(currentPage + 1)">
            <icon-mdi-chevron-right/>
          </button>
        </li>
        <li class="page last">
          <button type="button" :disabled="isLastPage" @click="goToPage(totalPages - 1)">
            <icon-mdi-page-last/>
          </button>
        </li>
      </ul>
    </div>
  </div>
</template>

<style lang="scss">
.table-container {
  @apply flex flex-col;

  .table-header {
    @apply flex flex-row justify-between items-end;

    .pageSizeSelector {
      .select {
        @apply mb-1 w-18 self-end;
      }
    }
  }

  .table-wrapper {
    @apply rounded shadow overflow-hidden;

    table.table {
      @apply w-full;

      thead {
        @apply bg-light-blue-600 shadow;

        tr {
          th {
            @apply px-6 py-2 text-sm tracking-wider uppercase text-white font-medium;

            &.sortable {
              @apply cursor-pointer;
            }

            > .wrapper {
              @apply flex flex-row justify-between items-center;

              .text {
                @apply flex-grow;
              }
            }
          }
        }
      }
      tbody {
        @apply divide-y divide-gray-200 shadow;

        tr {
          &.empty-message {
            td {
              @apply text-center rounded shadow-sm;
            }
          }

          &.expandable {
            @apply h-0 p-0 m-0 border-none overflow-hidden;

            td {
              @apply h-0 p-0 m-0 border-none overflow-hidden;
            }
          }

          td {
            @apply px-6 py-3 text-sm;
          }
        }
      }
    }
  }

  .pageable {
    @apply flex flex-row justify-between items-center mt-2;

    .legend {
      @apply text-sm text-gray-600;
    }

    .pagination {
      @apply flex flex-row shadow rounded-lg;

      .page {
        button {
          @apply h-7 min-w-7 border flex flex-col items-center justify-center text-gray-700 text-xs;

          &:disabled {
            @apply bg-gray-200 bg-opacity-10 text-gray-400 cursor-not-allowed shadow-inner;
          }
        }

        &.active {
          button {
            @apply bg-light-blue-500 text-white border-light-blue-500;
          }
        }

        &.first {
          button {
            @apply rounded-l-lg pl-0.5;
          }
        }
        &.last {
          button {
            @apply rounded-r-lg pr-0.5;
          }
        }
      }
    }
  }
}
</style>
