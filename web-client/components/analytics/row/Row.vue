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
  showTable: {
    type: Boolean,
    required: false,
    default: true,
  },
})

const { t } = useI18n()
const constants = useConstantsStore()
const dataStore = useDataStore()

const deviceStates = ref({})

const colors = {
  1: '#fdb0b0',
  2: '#fde0b0',
  3: '#e0B0fd',
  4: '#b2b0fd',
  5: '#b0fdbc',
  6: '#b0effd',
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

const devicesForTable = computed(() => {
  return [...props.devices]
    .filter(device => deviceStates.value[device.id])
    .sort((a, b) => (a['external_id'] > b['external_id'] ? 1 : -1))
})

const averageValue = computed(() => {
  const kind = props.devices[0]?.kind
  const kinds = constants.DEVICE_KINDS
  let value = 0
  let numberOfValues = 0

  for (const device of [...props.devices].filter(
    device => deviceStates.value[device.id]
  )) {
    const deviceRecords =
      dataStore.deviceRecordsAverage[props.range] === undefined
        ? undefined
        : dataStore.deviceRecordsAverage[props.range][device.id]

    if (deviceRecords === undefined) return

    for (const record of deviceRecords.records) {
      if (record.data === null) continue

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

  deviceStates.value = {}

  for (const device of [...props.devices].sort((a, b) =>
    a['external_id'] > b['external_id'] ? 1 : -1
  )) {
    deviceStates.value[device.id] = device.status === 1
  }
})

watchEffect(() => {
  const states = Object.values(deviceStates.value)

  if (states.length === 0) return

  if (states.filter(state => state).length === 0)
    deviceStates.value[Object.keys(deviceStates.value)[0]] = true
})
</script>

<template>
  <div class="row" :class="{ reversed, ['show-table']: showTable }">
    <div class="chart-container">
      <AnalyticsRowChart :devices="devicesForChart" :range="range" />
    </div>
    <div class="meta">
      <h4>{{ heading }}</h4>
      <div class="average">
        <span class="key">{{ t('average') }}</span>
        <span class="value">{{ averageValue }}</span>
      </div>
      <div class="devices">
        <div v-for="group in 2" :key="`group-${group}`" class="group">
          <div
            v-for="device in group === 1
              ? sortedDevices.slice(0, devices.length / 2)
              : sortedDevices.slice(devices.length / 2)"
            :key="`device-${device.id}`"
            class="device"
            :class="{ disabled: !deviceStates[device.id] }"
          >
            <GarthenCheckbox
              :checked="deviceStates[device.id]"
              @update:checked="state => (deviceStates[device.id] = state)"
            />
            <Transition enter-from-class="hide" leave-to-class="hide">
              <div
                v-if="deviceStates[device.id] && !showTable"
                class="dot"
                :style="`background: ${device.color};`"
              />
            </Transition>
            <p
              :class="{
                ['extra-margin']: !deviceStates[device.id] && !showTable,
              }"
              @click="deviceStates[device.id] = !deviceStates[device.id]"
            >
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
    <div class="table-container">
      <AnalyticsRowTable :devices="devicesForTable" :range="range" />
    </div>
  </div>
</template>

<style lang="scss" scoped>
.row {
  position: relative;
  display: flex;
  flex-direction: column-reverse;
  align-items: center;
  gap: 1.5rem;
  width: 100%;

  @include medium-screen {
    gap: 2rem;
  }

  @include xxl-screen {
    flex-direction: row;

    &.reversed {
      flex-direction: row-reverse;

      .table-container {
        left: 0;
      }
    }

    &.show-table {
      &.reversed .meta {
        transform: translateX(100%);
      }

      .meta {
        transform: translateX(-100%);
      }
    }
  }

  &.reversed .table-container {
    transform: translateX(-100%);
  }

  &.show-table {
    &.reversed {
      .chart-container {
        transform: translateX(100%);
      }

      .table-container {
        transform: translateX(0);
      }
    }

    .chart-container {
      height: 16rem;
      opacity: 0;
      transform: translateX(-100%);
    }

    .table-container {
      opacity: 1;
      transform: translateX(0);
    }
  }

  .chart-container {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 16rem;
    transition: var(--default-transition);

    @include xxl-screen {
      height: 25.25rem;
    }
  }

  .meta {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 1.5rem;
    width: 100%;
    height: calc(100% - 1rem * 2);
    text-align: center;
    transition: var(--default-transition);

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
            width: 0.5rem;
            height: 0.5rem;
            margin-right: -0.375rem;
            border-radius: 100%;
            transition: var(--fast-transition-duration);

            @include medium-screen {
              margin-right: -0.5rem;
            }

            &.hide {
              margin-right: calc(-0.5rem + -0.75rem);
              opacity: 0;

              @include medium-screen {
                margin-right: calc(-0.5rem + -1rem);
              }
            }
          }

          p {
            cursor: pointer;
            font-size: var(--medium-font-size);
            text-align: start;
            transition: var(--fast-transition-duration);

            &:hover {
              opacity: 0.9;
            }

            &:active {
              opacity: 0.5;
            }

            // TODO: Make it smoother
            // &.extra-margin {
            //   margin-right: calc(0.5rem + 0.75rem);
            //
            //   @include medium-screen {
            //     margin-right: calc(0.5rem + 0.5rem);
            //   }
            //}
          }
        }
      }
    }
  }

  .table-container {
    position: absolute;
    display: flex;
    align-items: self-start;
    bottom: 0;
    right: 0;
    width: 100%;
    height: 16rem;
    max-height: 16rem;
    border-radius: var(--xl-radius);
    opacity: 0;
    transform: translateX(100%);
    overflow: auto;
    scrollbar-width: none;
    transition: var(--default-transition);

    &::-webkit-scrollbar {
      display: none;
    }

    @include xxl-screen {
      align-items: initial;
      bottom: initial;
      width: calc(50% - 1rem);
      height: auto;
      max-height: 100%;
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
