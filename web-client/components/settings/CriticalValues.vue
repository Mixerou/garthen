<script setup>
const props = defineProps({
  disabled: {
    type: Boolean,
    required: false,
    default: false,
  },
  maximumAverageHumidity: {
    type: String,
    required: true,
  },
  minimumAverageTemperature: {
    type: String,
    required: true,
  },
})
const emit = defineEmits([
  'update:maximumAverageHumidity',
  'update:minimumAverageTemperature',
  'update:maximumSoilMoistureData',
])

const { t } = useI18n()
const route = useRoute()
const constants = useConstantsStore()
const dataStore = useDataStore()

const computedMaximumAverageHumidity = computed({
  get: () => {
    return props.maximumAverageHumidity
  },
  set: value => {
    if (props.disabled) return
    emit('update:maximumAverageHumidity', value)
  },
})

const computedMinimumAverageTemperature = computed({
  get: () => {
    return props.minimumAverageTemperature
  },
  set: value => {
    if (props.disabled) return
    emit('update:minimumAverageTemperature', value)
  },
})

const devices = computed(() => {
  return Object.values(dataStore.devices)
    .filter(
      device =>
        device['greenhouse_id'] === BigInt(route.params.greenhouseId) &&
        device.kind === constants.DEVICE_KINDS.soilMoistureSensor
    )
    .sort((a, b) => (a['external_id'] > b['external_id'] ? 1 : -1))
})
</script>

<template>
  <section>
    <h5>{{ t('heading') }}</h5>

    <div class="groups">
      <div class="group">
        <p class="key">{{ t('keys.minimumAverageTemperature') }}</p>
        <GarthenInput
          v-model:text="computedMinimumAverageTemperature"
          :disabled="disabled"
          :placeholder="t('warning')"
          max-length="4"
        />
      </div>
      <div class="group">
        <p class="key">{{ t('keys.maximumAverageHumidity') }}</p>
        <GarthenInput
          v-model:text="computedMaximumAverageHumidity"
          :disabled="disabled"
          :placeholder="t('warning')"
          max-length="4"
        />
      </div>
      <div class="group">
        <p class="key">{{ t('keys.maximumSoilMoisture') }}</p>
      </div>
      <div v-for="device in devices" :key="`device-${device.id}`" class="group">
        <p class="key">
          {{
            device.name === null
              ? $t(`defaultDeviceNames.${device.kind}`, {
                  externalId: device['external_id'],
                })
              : device.name
          }}
        </p>
        <GarthenInput
          :text="device['maximum_data_value']"
          :disabled="disabled"
          :placeholder="t('warning')"
          max-length="4"
          @update:text="
            value =>
              emit('update:maximumSoilMoistureData', {
                id: device.id,
                data: value,
              })
          "
        />
      </div>
    </div>
  </section>
</template>

<style lang="scss" scoped></style>

<i18n lang="json">
{
  "en-GB": {
    "heading": "Critical values",
    "warning": "It is not advisable to leave it empty",
    "keys": {
      "minimumAverageTemperature": "Minimum average air temperature",
      "maximumAverageHumidity": "Maximum average air humidity",
      "maximumSoilMoisture": "Maximum soil moisture"
    }
  },
  "ru-RU": {
    "heading": "Критические значения",
    "warning": "Не рекомендуется оставлять пустым",
    "keys": {
      "minimumAverageTemperature": "Минимальная средняя температура воздуха",
      "maximumAverageHumidity": "Максимальная средняя влажность воздуха",
      "maximumSoilMoisture": "Максимальная влажность почвы"
    }
  }
}
</i18n>
