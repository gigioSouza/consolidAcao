<script lang="ts" setup>
import { useVuelidate } from '@vuelidate/core';

const props = withDefaults(defineProps<{
  rules: Object,
  state: Object,
  options?: Object
}>(), {
  options: { $scope: true }
});

const v = useVuelidate(props.rules, props.state, props.options);
</script>

<template>
  <slot :v="v"/>
</template>
