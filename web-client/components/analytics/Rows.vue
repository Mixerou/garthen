<script setup>
const props = defineProps({
  range: {
    type: Number,
    required: true,
  },
})

const { t } = useI18n()
const route = useRoute()
const constants = useConstantsStore()
const dataStore = useDataStore()

const devices = computed(() => {
  const greenhouseId =
    dataStore.greenhouses[BigInt(route.params.greenhouseId)]?.id
  const devices = {
    humidity: [],
    soilMoisture: [],
    temperature: [],
  }

  for (const device of Object.values(dataStore.devices)) {
    if (device['greenhouse_id'] === greenhouseId) {
      if (device.kind === constants.DEVICE_KINDS.humiditySensor)
        devices.humidity.push(device)
      else if (device.kind === constants.DEVICE_KINDS.soilMoistureSensor)
        devices.soilMoisture.push(device)
      else if (device.kind === constants.DEVICE_KINDS.temperatureSensor)
        devices.temperature.push(device)
    }
  }

  return devices
})
</script>

<template>
  <section>
    <AnalyticsRow
      :heading="t('headings.humidity')"
      :devices="devices.humidity"
      :range="range"
    />
    <AnalyticsRow
      :heading="t('headings.temperature')"
      :devices="devices.temperature"
      :range="range"
      reversed
    />
    <AnalyticsRow
      :heading="t('headings.soilMoisture')"
      :devices="devices.soilMoisture"
      :range="range"
    />
  </section>
</template>

<style lang="scss" scoped>
section {
  display: flex;
  flex-direction: column;
  flex-wrap: wrap;
  align-items: center;
  gap: 2rem;
  width: 100%;
  height: 100%;

  @include xxl-screen {
    width: calc(100% - 1rem * 2);
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "headings": {
      "humidity": "Humidity sensors",
      "soilMoisture": "Soil moisture",
      "temperature": "Temperature sensors"
    }
  },
  "ru-RU": {
    "headings": {
      "humidity": "Датчики влажности",
      "soilMoisture": "Влажность почвы",
      "temperature": "Датчики температуры"
    }
  }
}
</i18n>
