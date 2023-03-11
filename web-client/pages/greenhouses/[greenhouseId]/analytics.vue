<script setup>
definePageMeta({
  layout: 'app',
})

const { $wsSubscribe } = useNuxtApp()
const { t } = useI18n()
const route = useRoute()
const constants = useConstantsStore()
const dataStore = useDataStore()
const system = useSystemStore()

const isLoading = ref(false)
const selectedRange = ref(constants.DEVICE_RECORDS_TIMESTAMP_RANGES.today)

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

const selectRange = (range, isForce = false) => {
  if (!isForce && (isLoading.value || selectedRange.value === range)) return

  isLoading.value = true
  selectedRange.value = range

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

watchEffect(() => {
  try {
    useHead({
      title: t('fullTitle', {
        greenhouseName:
          dataStore.greenhouses[BigInt(route.params.greenhouseId)].name,
      }),
    })
  } catch (e) {
    useHead({ title: t('title') })
  }
})

watchEffect(() => {
  $wsSubscribe(
    'devices',
    {
      a: 'subscribe_to_devices_update',
      greenhouse_id: BigInt(route.params.greenhouseId || 0),
    },
    true
  )
})

onMounted(() => {
  system.setAppPageName('analytics')

  watchEffect(() => {
    for (const device of devices.value) {
      $wsSubscribe(
        'device_records/average',
        {
          a: 'subscribe_to_device_records_average_update',
          ['device_id']: device.id,
          ['greenhouse_id']: BigInt(route.params.greenhouseId || 0),
          range: selectedRange.value,
        },
        true
      )
    }
  })
})

onBeforeUnmount(() => system.setAppPageName(''))
</script>

<template>
  <div class="page">
    <AnalyticsHeader
      :loading="isLoading"
      :range="selectedRange"
      @select:range="selectRange"
    />

    <AnalyticsRows :range="selectedRange" />
  </div>
</template>

<style lang="scss" scoped>
.page {
  display: flex;
  flex-direction: column;
  gap: 1.75rem;
  margin: 1.5rem 0 4.5rem 0;

  @include medium-screen {
    gap: 4rem;
    margin: 2.125rem 0;
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "title": "Analytics",
    "fullTitle": "Analytics - {greenhouseName}"
  },
  "ru-RU": {
    "title": "Аналитика",
    "fullTitle": "Аналитика - {greenhouseName}"
  }
}
</i18n>
