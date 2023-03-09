<script setup>
const { $wsSubscribe } = useNuxtApp()
const route = useRoute()
const constants = useConstantsStore()
const dataStore = useDataStore()

const isLoading = ref(false)

const devices = computed(() => {
  const greenhouseId =
    dataStore.greenhouses[BigInt(route.params.greenhouseId)]?.id
  let devices = []

  for (const device of Object.values(dataStore.devices)) {
    if (
      device['greenhouse_id'] === greenhouseId &&
      !(
        device.kind === constants.DEVICE_KINDS.humidificationController ||
        device.kind === constants.DEVICE_KINDS.irrigationController ||
        device.kind === constants.DEVICE_KINDS.windowsController
      )
    ) {
      devices.push(device)
    }
  }

  return devices
})

const selectRange = range => {
  if (isLoading.value) return

  isLoading.value = true

  for (const device of devices.value) {
    $wsSubscribe(
      'device_records/average',
      {
        a: 'subscribe_to_device_records_average_update',
        ['device_id']: device.id,
        ['greenhouse_id']: BigInt(route.params.greenhouseId || 0),
        range,
      },
      true
    )
  }

  setTimeout(() => (isLoading.value = false), 1e3)
}
</script>

<template>
  <header>
    <AnalyticsHeaderLeftSide :loading="isLoading" @select:range="selectRange" />
    <AnalyticsHeaderRightSide />
  </header>
</template>

<style lang="scss" scoped>
header {
  display: flex;
  justify-content: space-between;
  gap: 0.25rem;
}
</style>
