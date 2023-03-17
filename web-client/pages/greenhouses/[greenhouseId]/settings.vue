<!-- TODO: Rewrite this page (including its components) -->
<script setup>
import IconClose from '@/assets/icons/close.svg?skipsvgo'

definePageMeta({
  layout: 'app',
})

const { $wsSendAndWait, $wsSubscribe } = useNuxtApp()
const { t } = useI18n()
const route = useRoute()
const constants = useConstantsStore()
const dataStore = useDataStore()
const system = useSystemStore()
const user = useUserStore()

const greenhouseName = ref('')
const greenhouseToken = ref('')
const maximumAverageHumidity = ref('')
const minimumAverageTemperature = ref('')
const soilMoistureData = ref({})

const error = ref('')

const isError = ref(false)
const isSaving = ref(false)
const isDeletionModalOpened = ref(false)

const save = async () => {
  isSaving.value = true

  const greenhouseData = {
    a: 'request_patch_greenhouse',
    id: BigInt(route.params.greenhouseId || 0),
    name: greenhouseName.value,
    token: greenhouseToken.value,
  }

  if (maximumAverageHumidity.value !== '') {
    let parsedData = parseFloat(
      maximumAverageHumidity.value.replaceAll(',', '.')
    )
    greenhouseData['maximum_average_humidity'] = isNaN(parsedData)
      ? 80.0
      : parsedData
  }

  if (minimumAverageTemperature.value !== '') {
    let parsedData = parseFloat(
      minimumAverageTemperature.value.replaceAll(',', '.')
    )
    greenhouseData['minimum_average_temperature'] = isNaN(parsedData)
      ? 21.0
      : parsedData
  }

  let greenhouseResponse = await $wsSendAndWait({
    o: 2,
    r: 'greenhouse',
    m: { patch: null },
    d: greenhouseData,
  })

  if (greenhouseResponse.d.code !== 200) {
    isError.value = true
    error.value = constants.parseGlobalWsErrorCode(greenhouseResponse.d.code)
  } else {
    isError.value = false
  }

  if (!isError.value) {
    for (const deviceId of Object.keys(soilMoistureData.value)) {
      const deviceData = {
        a: 'request_patch_device',
        id: BigInt(deviceId),
        greenhouse_id: BigInt(route.params.greenhouseId),
        name: dataStore.devices[deviceId].name,
      }

      if (soilMoistureData.value[deviceId] !== '') {
        let parsedData = parseFloat(
          soilMoistureData.value[deviceId].replaceAll(',', '.')
        )
        deviceData['maximum_data_value'] = isNaN(parsedData) ? 80.0 : parsedData
      }

      let deviceResponse = await $wsSendAndWait({
        o: 2,
        r: 'device',
        m: { patch: null },
        d: deviceData,
      })

      if (deviceResponse.d.code !== 200) {
        isError.value = true
        error.value = constants.parseGlobalWsErrorCode(deviceResponse.d.code)
      }
    }
  }

  soilMoistureData.value = {}
  isSaving.value = false
}

const reset_device_names = async () => {
  isSaving.value = true

  await $wsSendAndWait({
    o: 2,
    r: 'devices/reset-names',
    m: { patch: null },
    d: {
      a: 'request_patch_devices_reset_names',
      ['greenhouse_id']: BigInt(route.params.greenhouseId || 0),
    },
  })

  isSaving.value = false
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
  try {
    const greenhouse = dataStore.greenhouses[BigInt(route.params.greenhouseId)]

    greenhouseName.value = greenhouse.name
    greenhouseToken.value = greenhouse.token
    maximumAverageHumidity.value = String(
      greenhouse['maximum_average_humidity'] || ''
    )
    minimumAverageTemperature.value = String(
      greenhouse['minimum_average_temperature'] || ''
    )
  } catch {}
})

onMounted(() => {
  system.setAppPageName('settings')

  $wsSubscribe(
    'devices',
    {
      a: 'subscribe_to_devices_update',
      greenhouse_id: BigInt(route.params.greenhouseId || 0),
    },
    true
  )
})

onBeforeUnmount(() => system.setAppPageName(''))
</script>

<template>
  <div class="page">
    <SettingsDeleteGreenhouseModal
      v-if="isDeletionModalOpened"
      @close="isDeletionModalOpened = false"
    />

    <SettingsHeader :saving="isSaving" @save="save" />

    <div class="sections">
      <div class="row">
        <SettingsGeneral
          :disabled="isSaving"
          :greenhouse-name="greenhouseName"
          :greenhouse-token="greenhouseToken"
          @update:greenhouse-name="name => (greenhouseName = name)"
          @update:greenhouse-token="token => (greenhouseToken = token)"
        />
        <SettingsCriticalValues
          :maximum-average-humidity="maximumAverageHumidity"
          :minimum-average-temperature="minimumAverageTemperature"
          @update:maximum-average-humidity="
            value => (maximumAverageHumidity = value)
          "
          @update:minimum-average-temperature="
            value => (minimumAverageTemperature = value)
          "
          @update:maximum-soil-moisture-data="
            value => (soilMoistureData[value.id] = value.data)
          "
        />
      </div>
      <div class="row">
        <section class="empty" />
        <SettingsDangerZone
          :disabled="isSaving"
          @reset-device-names="reset_device_names"
          @delete="isDeletionModalOpened = true"
        />
      </div>
    </div>

    <Transition enter-from-class="hide" leave-to-class="hide">
      <div v-if="isError && error !== ''" class="error">
        <Transition enter-from-class="hide" leave-to-class="hide" mode="out-in">
          <p :key="`error-${error}`">{{ $t(`globalWsErrors.${error}`) }}</p>
        </Transition>
        <IconClose @click="isError = false" />
      </div>
    </Transition>
  </div>
</template>

<style lang="scss" scoped>
.page {
  display: flex;
  flex-direction: column;
  gap: 1.75rem;
  margin: 1.5rem 0 4rem 0;

  @include medium-screen {
    gap: 4rem;
    margin: 2.125rem 0;
  }

  .sections {
    display: flex;
    flex-direction: column;
    gap: 2rem;

    .row {
      display: flex;
      flex-direction: column;
      gap: 1.5rem;
      width: 100%;

      @include medium-screen {
        flex-direction: row;
        gap: 2rem;
      }

      :deep(section) {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        width: 100%;

        &.empty {
          margin-bottom: -1.5rem;
          pointer-events: none;

          @include medium-screen {
            margin-bottom: -2rem;
          }
        }

        .groups {
          display: flex;
          flex-direction: column;
          gap: 1rem;

          .group {
            display: flex;
            flex-direction: column;
            gap: 0.5rem;

            .key {
              font-size: var(--default-font-size);
              font-weight: 600;
            }
          }
        }
      }
    }
  }

  .error {
    position: fixed;
    display: flex;
    justify-content: space-between;
    align-items: center;
    bottom: 1rem;
    right: 1rem;
    width: calc(40rem - 2rem * 2);
    max-width: calc(100vw - 2rem * 2 - 6.5rem);
    height: 3.5rem;
    padding: 0 1.5rem;
    border-radius: var(--large-radius);
    box-shadow: var(--large-shadow);
    background: var(--primary-layer-0-background);
    color: var(--primary-layer-0-color);
    fill: var(--primary-layer-0-color);
    transition: var(--default-transition);

    @include medium-screen {
      bottom: 1.5rem;
      height: 4rem;
    }

    &.hide {
      box-shadow: none;
      transform: translateY(calc(100% + 1rem + 4rem));
    }

    p {
      font-size: var(--default-font-size);
      transition: var(--fast-transition-duration);

      &.hide {
        opacity: 0;
      }
    }

    svg {
      width: 1.5rem;
      opacity: 0.75;
      cursor: pointer;
      transition: var(--fast-transition-duration);

      &:hover {
        opacity: 0.9;
      }

      &:active {
        opacity: 0.5;
      }
    }
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "title": "Settings",
    "fullTitle": "Settings - {greenhouseName}"
  },
  "ru-RU": {
    "title": "Настройки",
    "fullTitle": "Настройки - {greenhouseName}"
  }
}
</i18n>
