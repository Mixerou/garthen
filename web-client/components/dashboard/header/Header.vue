<script setup>
const isRefreshing = ref(false)

const refreshData = () => {
  if (isRefreshing.value) return

  setTimeout(() => (isRefreshing.value = true), 1)
  setTimeout(() => (isRefreshing.value = false), 3e3)
}
</script>

<template>
  <header>
    <DashboardHeaderLeftSide
      :refreshing="isRefreshing"
      @refresh="refreshData"
    />
    <DashboardHeaderRightSide />
  </header>
</template>

<style lang="scss" scoped>
header {
  display: flex;
  justify-content: space-between;
  gap: 0.25rem;
}
</style>
