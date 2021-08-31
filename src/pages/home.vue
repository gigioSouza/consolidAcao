<route>
name: home
meta:
  title: Home
  layout: MenuLayout
</route>

<script setup>

</script>

<template>
  <h1>Home</h1>
  <form>
    <div class="field">
      <label>Select</label>
      <select tabindex="1">
        <option value="1">1</option>
        <option value="2">2</option>
      </select>
    </div>
    <div class="field">
      <label>Input text</label>
      <input type="text" tabindex="2"/>
    </div>
    <div class="field">
      <label>Input radio</label>
      <label for="radio">
        <input id="radio" type="radio" tabindex="3"/>
      </label>
    </div>
    <div class="field">
      <label>Input checkbox</label>
      <label for="checkbox">
        <input id="checkbox" type="checkbox" tabindex="4"/>
      </label>
    </div>
    <button tabindex="6">Button</button>
  </form>
</template>

<style lang="scss" scoped>
select,
input,
button {
  &:focus {
    @apply outline-solid-red-500;
  }
}
</style>
