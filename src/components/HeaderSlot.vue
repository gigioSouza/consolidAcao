<script lang="ts" setup>
import { onMounted, ref } from 'vue';

const isMounted = ref(false);

onMounted(() => isMounted.value = true);
</script>

<template>
  <teleport v-if="isMounted" to="body #app .layout .contentContainer .header #header-slot">
    <slot/>
  </teleport>
</template>
