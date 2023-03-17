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
])

const { t } = useI18n()

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
          max-length="32"
        />
        <div class="group">
          <p class="key">{{ t('keys.maximumAverageHumidity') }}</p>
          <GarthenInput
            v-model:text="computedMaximumAverageHumidity"
            :disabled="disabled"
            :placeholder="t('warning')"
            max-length="32"
          />
        </div>
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
      "maximumAverageHumidity": "Maximum average air humidity"
    }
  },
  "ru-RU": {
    "heading": "Критические значения",
    "warning": "Не рекомендуется оставлять пустым",
    "keys": {
      "minimumAverageTemperature": "Минимальная средняя температура воздуха",
      "maximumAverageHumidity": "Максимальная средняя влажность воздуха"
    }
  }
}
</i18n>
