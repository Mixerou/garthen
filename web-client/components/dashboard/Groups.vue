<script setup>
const { t } = useI18n()
const route = useRoute()
const dataStore = useDataStore()

const exampleGroups = [
  {
    name: 'general',
    kinds: [3, 5],
  },
  {
    name: 'humidity',
    kinds: [0],
  },
  {
    name: 'temperature',
    kinds: [2],
  },
  {
    name: 'moisture',
    kinds: [1],
  },
  {
    name: 'irrigation',
    kinds: [4],
  },
]

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
    .filter(device => device.status !== 2)
    .sort((a, b) => (a['external_id'] < b['external_id'] ? 1 : -1))
    .sort((a, b) => (a.kind > b.kind ? 1 : -1))
})
</script>

<template>
  <section>
    <DashboardGroup
      v-for="(group, index) in exampleGroups"
      :key="`group-${index}`"
      :name="t(`exampleGroupNames.${group.name}`)"
    >
      <TransitionGroup enter-from-class="hide" leave-to-class="hide">
        <DashboardCard
          v-for="device in devices.filter(device =>
            group.kinds.includes(device.kind)
          )"
          :id="device.id"
          :key="`device-${device.id}`"
          :external-id="device['external_id']"
          :name="device.name"
          :kind="device.kind"
          :value="device['latest_data']"
        />
      </TransitionGroup>
    </DashboardGroup>
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

  @include medium-screen {
    flex-direction: row;
    justify-content: center;
    width: calc(100% - 1rem * 2);
  }

  .hide {
    @include medium-screen {
      margin-right: calc(-14rem - 4rem);
      opacity: 0;
    }
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "exampleGroupNames": {
      "general": "General",
      "humidity": "Humidity",
      "temperature": "Temperature",
      "moisture": "Moisture",
      "irrigation": "Irrigation"
    }
  },
  "ru-RU": {
    "exampleGroupNames": {
      "general": "Общее",
      "humidity": "Влажность в теплице",
      "temperature": "Температура в теплице",
      "moisture": "Влажность почвы",
      "irrigation": "Полив"
    }
  }
}
</i18n>
