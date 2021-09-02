<route>
name: brokerage-edit
meta:
  title: Editar Nota de Corretagem
  layout: MenuLayout
</route>

<script lang="ts" setup>
import { useRoute } from 'vue-router';
import { useNote } from './composables/note';

const route = useRoute();
const { note } = useNote(+route.params.id);
</script>

<template>
  <div>
    <h1>{{ route.params.id }}</h1>
    <pre>{{ note }}</pre>
  </div>
</template>

<style scoped>

</style>
