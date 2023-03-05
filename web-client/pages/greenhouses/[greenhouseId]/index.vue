<script setup>
definePageMeta({
  layout: 'app',
})

const system = useSystemStore()

onMounted(() => system.setAppPageName('dashboard'))
onBeforeUnmount(() => system.setAppPageName(''))
</script>

<template>
  <div class="page">
    <h3>Welcome back, John!</h3>
  </div>
</template>

<style lang="scss" scoped>
.page {
  margin-top: 1.25rem;
}
</style>
