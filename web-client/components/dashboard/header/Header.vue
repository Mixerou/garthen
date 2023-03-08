<script setup>
const { $wsSendAndWait } = useNuxtApp()
const route = useRoute()

const isRefreshing = ref(false)

const refreshData = async () => {
  if (isRefreshing.value) return

  isRefreshing.value = true

  await $wsSendAndWait({
    o: 2,
    r: 'device/request-data',
    m: { post: null },
    d: {
      a: 'request_post_device_request_data',
      greenhouse_id: BigInt(route.params.greenhouseId),
    },
  })

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
