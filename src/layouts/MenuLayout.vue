<template>
  <div class="layout">
    <nav class="menu">
      <router-link as="h2" :to="{ name: 'home' }">ConsolidAção</router-link>
    </nav>
    <div class="contentContainer">
      <div class="header">
        <h1 class="title">{{ route.meta.title }}</h1>
      </div>
      <div class="content">
        <router-view/>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useRoute } from 'vue-router';

const route = useRoute();
</script>

<style lang="scss" scoped>
.layout {
  @apply w-screen h-screen flex flex-row bg-gray-100;

  .menu {
    width: 250px;
    @apply bg-light-blue-700;
  }

  .contentContainer {
    @apply flex-grow flex flex-col;

    .header {
      height: 60px;
      @apply p-4 bg-white flex flex-row justify-between shadow;

      .title {
        @apply text-gray-700 text-xl;
      }
    }
  }
}
</style>
