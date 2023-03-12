<script setup>
const emit = defineEmits(['close'])

const { $wsSendAndWait } = useNuxtApp()
const { t } = useI18n()
const i18n = useI18n()
const route = useRoute()
const constants = useConstantsStore()
const dataStore = useDataStore()
const system = useSystemStore()

const isLoading = ref(false)
const isError = ref(false)
const error = ref('')

const data = ref('')
const date = ref('')
const isDataIncorrect = ref(false)
const isDateIncorrect = ref(false)

const selectedDevice = ref(1)

const devices = computed(() => {
  const greenhouseId =
    dataStore.greenhouses[BigInt(route.params.greenhouseId)]?.id
  let earlyDevices = []
  let devices = []

  for (const device of Object.values(dataStore.devices)) {
    if (device['greenhouse_id'] === greenhouseId) {
      earlyDevices.push(device)
    }
  }

  earlyDevices
    .filter(
      device =>
        device.status !== 2 &&
        device.kind !== constants.DEVICE_KINDS.humidificationController &&
        device.kind !== constants.DEVICE_KINDS.irrigationController &&
        device.kind !== constants.DEVICE_KINDS.windowsController
    )
    .sort((a, b) => (a['external_id'] > b['external_id'] ? 1 : -1))
    .map(device =>
      devices.push({
        id: device.id,
        value:
          device.name === null
            ? i18n.t(`defaultDeviceNames.${device.kind}`, {
                externalId: device['external_id'],
              })
            : device.name,
      })
    )

  if (devices.filter(device => selectedDevice.value === device.id).length === 0)
    selectedDevice.value = devices[0]?.id

  return devices
})

const add = () => {
  isDataIncorrect.value = false
  isDateIncorrect.value = false

  setTimeout(async () => {
    if (data.value === '') isDataIncorrect.value = true
    if (date.value === '') isDateIncorrect.value = true

    let parsedData = parseFloat(data.value.replaceAll(',', '.'))

    if (isNaN(parsedData)) isDataIncorrect.value = true
    if (isDataIncorrect.value || isDateIncorrect.value) return

    isLoading.value = true

    let mappedDate = date.value.split(/\D/).map(sign => parseInt(sign))

    mappedDate[1] = mappedDate[1] - 1

    const response = await $wsSendAndWait({
      o: 2,
      r: 'device/custom-data',
      m: { post: null },
      d: {
        a: 'request_post_device_custom_data',
        id: selectedDevice.value,
        ['greenhouse_id']: BigInt(route.params.greenhouseId),
        data: parsedData,
        time: new Date(...mappedDate).getTime() / 1000,
      },
    })

    console.log(response)

    if (response?.d?.code !== 200) {
      isError.value = true
      error.value = constants.parseGlobalWsErrorCode(response.d.code)
      isLoading.value = false

      return
    }

    isError.value = false
    isLoading.value = false

    // TODO: Make it safe
    system.unregisterActiveModal()
  })
}
</script>

<template>
  <GarthenModal
    close-on-click-outside
    :is-dropdown-visible="isError"
    @close="emit('close')"
  >
    <template #dropdown>
      <Transition enter-from-class="hide" leave-to-class="hide" mode="out-in">
        <span :key="`error-${error}`">
          {{ $t(`globalWsErrors.${error}`) }}
        </span>
      </Transition>
    </template>

    <template #content>
      <h5>{{ t('heading') }}</h5>

      <div class="information">
        <GarthenInput
          :text="data"
          type="text"
          :disabled="isLoading"
          :error="isDataIncorrect"
          :placeholder="t('inputs.data')"
          @update:text="value => (data = value)"
        />
        <GarthenInput
          :text="date"
          type="datetime-local"
          :disabled="isLoading"
          :error="isDateIncorrect"
          @update:text="value => (date = value)"
        />
        <GarthenSelect
          :disabled="isLoading"
          :items="devices"
          :selected="selectedDevice"
          @update:selected="value => (selectedDevice = value)"
        />
      </div>

      <GarthenButton :loading="isLoading" @click="add">
        {{ $t('go') }}
      </GarthenButton>
    </template>
  </GarthenModal>
</template>

<style lang="scss" scoped>
.dropdown {
  span {
    transition-duration: var(--fast-transition-duration);

    &.hide {
      opacity: 0;
    }
  }
}

.content {
  .information {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;

    :deep(.select .options) {
      max-height: 6.25rem;
      overflow-y: auto;

      @include medium-screen {
        max-height: 7rem;
      }
    }
  }

  .hide {
    opacity: 0;
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "heading": "Adding",
    "inputs": {
      "data": "Data"
    }
  },
  "ru-RU": {
    "heading": "Добавление",
    "inputs": {
      "data": "Данные"
    }
  }
}
</i18n>
