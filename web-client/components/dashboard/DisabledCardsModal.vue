<script setup>
const emit = defineEmits(['close'])

const { $wsSend } = useNuxtApp()
const { t } = useI18n()
const route = useRoute()
const dataStore = useDataStore()

const isMounted = ref(false)
const isEnableAllButtonLoading = ref(false)

const devices = computed(() => {
  const greenhouseId =
    dataStore.greenhouses[BigInt(route.params.greenhouseId)]?.id
  let devices = []

  for (const device of Object.values(dataStore.devices)) {
    if (device['greenhouse_id'] === greenhouseId) {
      devices.push(device)
    }
  }

  return devices
    .filter(device => device.status === 2)
    .sort((a, b) => (a['external_id'] < b['external_id'] ? 1 : -1))
    .sort((a, b) => (a.kind > b.kind ? 1 : -1))
})

const enableAll = () => {
  isEnableAllButtonLoading.value = true

  for (const device of devices.value) {
    $wsSend({
      o: 2,
      r: 'device/enable',
      m: { post: null },
      d: {
        a: 'request_post_device_disable',
        id: device.id,
        ['greenhouse_id']: BigInt(route.params.greenhouseId || 0),
      },
    })
  }

  setTimeout(() => (isEnableAllButtonLoading.value = false), 300)
}

onMounted(() => {
  const timeoutTime =
    getComputedStyle(document.body)
      .getPropertyValue('--default-transition-duration')
      .split('s')[0] * 1000

  setTimeout(() => {
    isMounted.value = true
  }, timeoutTime / 2)
})
</script>

<template>
  <GarthenModal
    close-on-click-outside
    :is-dropdown-visible="devices.length === 0 && isMounted"
    @close="emit('close')"
  >
    <template #dropdown>
      <span>{{ t(`allDevicesEnabled`) }}</span>
    </template>

    <template #content>
      <h5>{{ t('heading') }}</h5>

      <TransitionGroup
        enter-from-class="hide"
        enter-active-class="transition"
        leave-to-class="hide"
        leave-active-class="transition"
      >
        <div v-if="devices.length !== 0" class="devices">
          <TransitionGroup
            enter-from-class="hide"
            enter-active-class="transition"
            leave-to-class="hide"
            leave-active-class="transition"
          >
            <DashboardCard
              v-for="device in devices"
              :id="device.id"
              :key="`device-${device.id}`"
              :external-id="device['external_id']"
              :name="device.name"
              :kind="device.kind"
              :value="device['latest_data']"
              disabled
            />
          </TransitionGroup>
        </div>

        <GarthenButton
          v-if="devices.length > 1"
          class="enable-all"
          :loading="isEnableAllButtonLoading"
          @click="enableAll"
        >
          {{ t('enableAllButton') }}
        </GarthenButton>
      </TransitionGroup>
    </template>
  </GarthenModal>
</template>

<style lang="scss" scoped>
.devices {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  overflow: hidden;

  .hide {
    margin-bottom: -13.5rem;
    opacity: 0;
  }

  .transition {
    transition: var(--default-transition);
  }
}

.hide {
  opacity: 0;

  &.devices {
    margin-bottom: -76%;

    @include medium-screen {
      margin-bottom: calc(-100% + 4.375rem);
    }
  }

  &.enable-all {
    margin-bottom: calc(-2.75rem - 1rem);
  }
}

.transition {
  transition: var(--default-transition);
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "heading": "Disabled devices",
    "allDevicesEnabled": "All devices are enabled!",
    "enableAllButton": "Enable all"
  },
  "ru-RU": {
    "heading": "Отключенные",
    "allDevicesEnabled": "Все устройства включены!",
    "enableAllButton": "Включить все"
  }
}
</i18n>
