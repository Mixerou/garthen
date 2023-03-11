<script setup>
const props = defineProps({
  heading: {
    type: String,
    required: true,
  },
  devices: {
    type: Array,
    required: true,
  },
  range: {
    type: Number,
    required: false,
    default: 0,
  },
  reversed: {
    type: Boolean,
    required: false,
    default: false,
  },
})

const { t } = useI18n()
const constants = useConstantsStore()
const dataStore = useDataStore()

const deviceStates = ref({})

const colors = {
  1: '#FDB0B0',
  2: '#FDE0B0',
  3: '#E0B0FD',
  4: '#B2B0FD',
  5: '#B0FDBC',
  6: '#B0EFFD',
}

const sortedDevices = computed(() =>
  [...props.devices].sort((a, b) =>
    a['external_id'] > b['external_id'] ? 1 : -1
  )
)

const devicesForChart = computed(() => {
  let devices = []

  for (let device of [...props.devices].filter(
    device => deviceStates.value[device.id]
  )) {
    device.color = colors[device['external_id']]

    devices.push(device)
  }

  return devices
})

const averageValue = computed(() => {
  const kind = props.devices[0]?.kind
  const kinds = constants.DEVICE_KINDS
  let value = 0
  let numberOfValues = 0

  for (const device of props.devices) {
    const deviceRecords =
      dataStore.deviceRecordsAverage[props.range] === undefined
        ? undefined
        : dataStore.deviceRecordsAverage[props.range][device.id]

    if (deviceRecords === undefined) return

    for (const record of deviceRecords.records) {
      value += record.data
      numberOfValues += 1
    }
  }

  value = Math.trunc((value / numberOfValues) * 100) / 100

  if (isNaN(value)) return '---'

  if (kind === kinds.humiditySensor || kind === kinds.soilMoistureSensor)
    return `${value}%`
  else if (kind === kinds.temperatureSensor) return `${value} °C`

  return value
})

watchEffect(() => {
  if (Object.keys(deviceStates.value).length === props.devices.length) return

  for (const device of props.devices) {
    deviceStates.value[device.id] = device.status === 1
  }
})
</script>

<template>
  <div class="row" :class="{ reversed }">
    <div class="chart-container">
      <AnalyticsChart :devices="devicesForChart" :range="range" />
    </div>
    <div class="meta">
      <h4>{{ heading }}</h4>
      <div class="average">
        <span class="key">{{ t('average') }}</span>
        <span class="value">{{ averageValue }}</span>
      </div>
      <div class="devices">
        <div class="group">
          <div
            v-for="device in sortedDevices.slice(0, devices.length / 2)"
            :key="`device-${device.id}`"
            class="device"
            :class="{ disabled: !deviceStates[device.id] }"
          >
            <GarthenCheckbox
              :checked="deviceStates[device.id]"
              @update:checked="state => (deviceStates[device.id] = state)"
            />
            <div class="dot" />
            <p @click="deviceStates[device.id] = !deviceStates[device.id]">
              {{
                device.name === null
                  ? $t(`defaultDeviceNames.${device.kind}`, {
                      externalId: device['external_id'],
                    })
                  : device.name
              }}
            </p>
          </div>
        </div>
        <div class="group">
          <div
            v-for="device in sortedDevices.slice(devices.length / 2)"
            :key="`device-${device.id}`"
            class="device"
            :class="{ disabled: !deviceStates[device.id] }"
          >
            <GarthenCheckbox
              :checked="deviceStates[device.id]"
              @update:checked="state => (deviceStates[device.id] = state)"
            />
            <div class="dot" />
            <p @click="deviceStates[device.id] = !deviceStates[device.id]">
              {{
                device.name === null
                  ? $t(`defaultDeviceNames.${device.kind}`, {
                      externalId: device['external_id'],
                    })
                  : device.name
              }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.row {
  display: flex;
  flex-direction: column-reverse;
  align-items: center;
  gap: 1rem;
  width: 100%;

  @include medium-screen {
    gap: 2rem;
  }

  @include xxl-screen {
    flex-direction: row;

    &.reversed {
      flex-direction: row-reverse;
    }
  }

  .chart-container {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;

    @include xxl-screen {
      height: 25.25rem;
    }
  }

  .meta {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 1rem;
    width: 100%;
    height: calc(100% - 1rem * 2);
    text-align: center;

    @include medium-screen {
      gap: 2rem;
    }

    @include xxl-screen {
      height: 25.25rem;
    }

    .average {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      padding: 0.25rem 0.75rem;
      border-radius: var(--large-radius);
      background: var(--primary-layer-0-background);
      font-size: var(--medium-font-size);
      font-weight: 600;
      color: var(--primary-layer-0-color);

      @include medium-screen {
        gap: 0.75rem;
        padding: 0.5rem 1rem;
      }

      .value {
        padding: 0.25rem 0.5rem;
        border-radius: var(--large-radius);
        background: var(--primary-layer-1-background);
        font-size: var(--default-font-size);
        color: var(--primary-layer-1-color);
      }
    }

    .devices {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      @include medium-screen {
        flex-direction: row;
        gap: 2rem;
      }

      .group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;

        .device {
          display: flex;
          align-items: center;
          gap: 0.75rem;

          @include medium-screen {
            gap: 1rem;
          }

          &.disabled p {
            opacity: 0.75;
          }

          .dot {
            display: none;
          }

          p {
            text-align: start;
            font-size: var(--medium-font-size);
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
    }
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "average": "Average"
  },
  "ru-RU": {
    "average": "Среднее"
  }
}
</i18n>
