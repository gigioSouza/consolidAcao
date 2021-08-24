<script setup>
import { useGlobalLoader } from '../store/global-loader';

const store = useGlobalLoader();
</script>

<template>
<div class="wrapper" v-if="store.isLoading">
  <i-eos-icons-loading class="loader animate-spin" />
</div>
</template>

<style lang="scss" scoped>
.wrapper {
  z-index: 100;
  @apply fixed top-0 left-0 h-screen w-screen flex flex-col items-center justify-center bg-cyan-100 bg-opacity-60;

  .loader {
    @apply text-7xl text-light-blue-700;
  }
}
</style>
