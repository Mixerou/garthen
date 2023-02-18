<script setup>
definePageMeta({
  layout: 'app',
})

const system = useSystemStore()

system.setIsNavbarFolded(true)
</script>

<template>
  <div class="content">
    <h1>App</h1>
  </div>
</template>

<style lang="scss" scoped>
.content {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 1rem;
  height: calc(100vh - 1rem * 2);
  padding: 1rem;

  h1 {
    opacity: 0.125;
  }
}
</style>
