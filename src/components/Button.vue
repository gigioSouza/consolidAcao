<script lang="ts" setup>
import { computed, defineAsyncComponent, useSlots, withDefaults } from 'vue';

const props = withDefaults(defineProps<{
  text?: string,
  icon?: string,
  variant?: string,
  submit?: boolean,
  reset?: boolean
}>(), {
  text: '',
  submit: false,
  reset: false
});

const slots = useSlots();

const icons = {
  save: defineAsyncComponent(() => import ('~icons/mdi/content-save')),
  back: defineAsyncComponent(() => import('~icons/mdi/undo-variant')),
  delete: defineAsyncComponent(() => import('~icons/mdi/trash-can')),
  plus: defineAsyncComponent(() => import('~icons/mdi/plus'))
}

const iconComponent = computed(() => {
  if (typeof props.icon === 'string') {
    return icons[props.icon];
  }
  return props.icon;
});

const buttonType = computed(() => {
  if (props.submit) return 'submit';
  if (props.reset) return 'reset';
  return 'button';
});
</script>

<template>
  <button :type="buttonType" class="button" :class="props.variant">
    <slot/>
    <component
      v-if="iconComponent != null"
      :is="iconComponent" :class="{
        icon: slots.default != null
      }"/>
  </button>
</template>

<style lang="scss" scoped>
@mixin button_focus($color) {
  &:not(&:disabled) {
    &:focus {
      box-shadow: 0 0 3pt 2pt $color;
    }
  }
}

.button {
  @apply rounded px-2 py-1 shadow-sm flex flex-row items-center justify-between outline-none select-none;

  &:hover,
  &:visited,
  &:focus {
    @apply outline-none;
  }

  &.light {
    @apply bg-gray-200 text-gray-800 font-light;
    @include button_focus($color: #a1a1aa);
  }

  &.primary {
    @apply bg-blue-600 text-white;
    @include button_focus($color: #1d4ed8);
  }

  &.secondary {
    @apply bg-emerald-500 text-white;
    @include button_focus($color: #059669);
  }

  &.danger {
    @apply bg-red-600 text-white;
    @include button_focus($color: #b91c1c);
  }

  &:disabled {
    @apply bg-opacity-30 cursor-not-allowed shadow-none;
  }

  &:hover {
    @apply shadow-md;
  }

  .icon {
    @apply ml-2;
  }
}
</style>
