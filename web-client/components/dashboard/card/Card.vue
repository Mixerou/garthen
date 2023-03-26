<script setup>
import IconSun from '@/assets/icons/sun.svg?skipsvgo'
import IconWaterDroplet from '@/assets/icons/water-droplet.svg?skipsvgo'
import IconWaterDrops from '@/assets/icons/water-drops.svg?skipsvgo'
import IconWaterFee from '@/assets/icons/water-fee.svg?skipsvgo'
import IconWateringCan from '@/assets/icons/watering-can.svg?skipsvgo'
import IconWindow from '@/assets/icons/window.svg?skipsvgo'

const props = defineProps({
  id: {
    type: BigInt,
    required: true,
  },
  externalId: {
    type: Number,
    required: true,
  },
  name: {
    type: String,
    required: true,
  },
  kind: {
    type: Number,
    required: true,
  },
  maximumDataValue: {
    type: Float32Array,
    required: true,
  },
  value: {
    type: Number,
    required: false,
    default: null,
  },
  disabled: {
    type: Boolean,
    required: false,
    default: false,
  },
  emergencyControl: {
    type: Boolean,
    required: true,
  },
})

const { t } = useI18n()
const i18n = useI18n()
const { $wsSend, $wsSendAndWait, $wsSubscribe } = useNuxtApp()
const route = useRoute()
const constants = useConstantsStore()
const dataStore = useDataStore()

const isActionButtonLoading = ref(false)
const isMainMenuOpened = ref(false)
const isEditMenuOpened = ref(false)

const isForceDisabled = computed(() => {
  if (props.emergencyControl) return false

  if (props.kind === constants.DEVICE_KINDS.humidificationController) {
    const devices = Object.values(dataStore.devices).filter(
      device =>
        device['greenhouse_id'] === BigInt(route.params.greenhouseId) &&
        device.kind === constants.DEVICE_KINDS.humiditySensor &&
        device['latest_data'] !== null
    )
    let data = 0

    devices.map(device => (data += device['latest_data']))

    return data === 0
      ? false
      : data / devices.length >=
          dataStore.greenhouses[BigInt(route.params.greenhouseId)][
            'maximum_average_humidity'
          ]
  } else if (props.kind === constants.DEVICE_KINDS.windowsController) {
    const devices = Object.values(dataStore.devices).filter(
      device =>
        device['greenhouse_id'] === BigInt(route.params.greenhouseId) &&
        device.kind === constants.DEVICE_KINDS.temperatureSensor &&
        device['latest_data'] !== null
    )
    let data = 0

    devices.map(device => (data += device['latest_data']))

    return data === 0
      ? false
      : data / devices.length <=
          dataStore.greenhouses[BigInt(route.params.greenhouseId)][
            'minimum_average_temperature'
          ]
  } else if (props.kind === constants.DEVICE_KINDS.irrigationController) {
    const device = Object.values(dataStore.devices).find(
      device =>
        device['greenhouse_id'] === BigInt(route.params.greenhouseId) &&
        device.kind === constants.DEVICE_KINDS.soilMoistureSensor &&
        device['external_id'] === props.externalId
    )
    if (device === undefined) return
    const latestData = device['latest_data']
    const maximumDataValue = device['maximum_data_value']

    return latestData === null || maximumDataValue === null
      ? false
      : latestData >= maximumDataValue
  }

  return false
})

const isController = computed(() => {
  return (
    props.kind === constants.DEVICE_KINDS.humidificationController ||
    props.kind === constants.DEVICE_KINDS.irrigationController ||
    props.kind === constants.DEVICE_KINDS.windowsController
  )
})

const bubblesColor = computed(() => {
  const kind = props.kind
  const kinds = constants.DEVICE_KINDS

  if (kind === kinds.humiditySensor || kind === kinds.soilMoistureSensor)
    return '#e5f1ff'
  else if (kind === kinds.temperatureSensor) return '#fde0b0'
  else if (kind === kinds.humidificationController) return '#b8e7ff'
  else if (kind === kinds.irrigationController) return '#cbf2a1'
  else if (kind === kinds.windowsController) return '#f1aaa7'

  return 'transparent'
})

const computedValue = computed(() => {
  const kind = props.kind
  const kinds = constants.DEVICE_KINDS
  const value = props.value

  if (value === null) return '---'

  if (kind === kinds.humiditySensor || kind === kinds.soilMoistureSensor)
    return `${value}%`
  else if (kind === kinds.temperatureSensor) return `${value} °C`

  return value
})

const computedName = computed({
  get() {
    return props.name === null
      ? i18n.t(`defaultDeviceNames.${props.kind}`, {
          externalId: props.externalId,
        })
      : props.name
  },
  set(value) {
    const data = {
      a: 'request_patch_device',
      id: props.id,
      greenhouse_id: BigInt(route.params.greenhouseId),
      name: value,
    }

    if (value === null || value === '') delete data.name
    if (props.maximumDataValue)
      data['maximum_data_value'] = props.maximumDataValue

    $wsSend({
      o: 2,
      r: 'device',
      m: { patch: null },
      d: data,
    })
  },
})

const changeState = async () => {
  isActionButtonLoading.value = true

  await $wsSendAndWait({
    o: 2,
    r: 'device/state',
    m: { patch: null },
    d: {
      a: 'request_patch_device_state',
      id: props.id,
      greenhouse_id: BigInt(route.params.greenhouseId),
      state: props.value === 1 ? 0 : 1,
    },
  })
}

const openCard = () => {
  setTimeout(() => {
    if (isActionButtonLoading.value || props.disabled) return
    if (!isController.value) {
      $wsSubscribe(
        'device_records',
        {
          a: 'subscribe_to_device_records_update',
          device_id: props.id,
          greenhouse_id: BigInt(route.params.greenhouseId || 0),
        },
        true
      )
    }

    isMainMenuOpened.value = true
  }, 10)
}

const closeCard = () => {
  isMainMenuOpened.value = false
  isEditMenuOpened.value = false
}

watchEffect(() => {
  props.value

  isActionButtonLoading.value = false
})

const disable = () => {
  setTimeout(() => closeCard(), 100)

  const timeoutTime =
    getComputedStyle(document.body)
      .getPropertyValue('--default-transition-duration')
      .split('s')[0] * 1000

  if (isController.value && props.value === 1) {
    $wsSend({
      o: 2,
      r: 'device/state',
      m: { patch: null },
      d: {
        a: 'request_patch_device_state',
        id: props.id,
        greenhouse_id: BigInt(route.params.greenhouseId),
        state: 0,
      },
    })
  }

  setTimeout(() => {
    $wsSend({
      o: 2,
      r: 'device/disable',
      m: { post: null },
      d: {
        a: 'request_post_device_disable',
        id: props.id,
        ['greenhouse_id']: BigInt(route.params.greenhouseId || 0),
      },
    })
  }, timeoutTime)
}

const enable = () => {
  $wsSend({
    o: 2,
    r: 'device/enable',
    m: { post: null },
    d: {
      a: 'request_post_device_disable',
      id: props.id,
      ['greenhouse_id']: BigInt(route.params.greenhouseId || 0),
    },
  })
}
</script>

<template>
  <div
    v-click-outside="closeCard"
    class="card"
    :class="{
      disabled,
      controller: isController,
      attention: isController && value === 1,
      ['main-menu-opened']: isMainMenuOpened,
      ['edit-menu-opened']: isEditMenuOpened,
    }"
    @click="openCard"
  >
    <div class="bubbles">
      <div class="small left" />
      <div class="medium left" />
      <div class="big">
        <IconWaterFee v-if="kind === constants.DEVICE_KINDS.humiditySensor" />
        <IconWaterDrops
          v-if="kind === constants.DEVICE_KINDS.soilMoistureSensor"
        />
        <IconSun
          v-else-if="kind === constants.DEVICE_KINDS.temperatureSensor"
        />
        <IconWaterDroplet
          v-else-if="kind === constants.DEVICE_KINDS.humidificationController"
        />
        <IconWateringCan
          v-else-if="kind === constants.DEVICE_KINDS.irrigationController"
        />
        <IconWindow
          v-else-if="kind === constants.DEVICE_KINDS.windowsController"
        />
      </div>
      <div class="medium right" />
      <div class="small right" />
    </div>
    <GarthenButton
      v-if="disabled"
      class="button"
      :loading="isActionButtonLoading"
      @click="enable"
    >
      {{ t('buttons.enable') }}
    </GarthenButton>
    <GarthenButton
      v-else-if="
        isController && kind === constants.DEVICE_KINDS.windowsController
      "
      class="button"
      :disabled="isForceDisabled && computedValue === 0"
      :loading="isActionButtonLoading"
      @click="changeState"
    >
      {{ computedValue === 1 ? t('buttons.close') : t('buttons.open') }}
    </GarthenButton>
    <GarthenButton
      v-else-if="isController"
      class="button"
      :disabled="isForceDisabled && computedValue === 0"
      :loading="isActionButtonLoading"
      @click="changeState"
    >
      {{ computedValue === 1 ? t('buttons.stop') : t('buttons.start') }}
    </GarthenButton>
    <p v-else class="value">{{ computedValue }}</p>
    <p class="name">
      {{ computedName }}
    </p>

    <DashboardCardInputs
      :name="name"
      :edit-menu-opened="isEditMenuOpened"
      :default-name="
        $t(`defaultDeviceNames.${kind}`, { externalId: externalId })
      "
      @update:name="value => (computedName = value)"
    />
    <DashboardCardStatistics
      :device-id="id"
      :edit-menu-opened="isEditMenuOpened"
      :value="computedValue"
      :controller="isController"
    />
    <DashboardCardButtons
      :main-menu-opened="isMainMenuOpened"
      :edit-menu-opened="isEditMenuOpened"
      @open:edit-menu="isEditMenuOpened = true"
      @close:edit-menu="isEditMenuOpened = false"
      @disable="disable"
    />
  </div>
</template>

<style lang="scss" scoped>
.card {
  position: relative;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 1rem;
  width: calc(100% - 1rem * 2);
  height: calc(12.5rem - 2rem * 2);
  padding: 2rem 1rem;
  border: 0.0625rem solid #1d313666;
  border-radius: var(--xl-radius);
  overflow: hidden;
  cursor: pointer;
  transition: var(--fast-transition-duration);

  &:not(.controller):not(.disabled) {
    &:hover {
      border-color: transparent;
      background: var(--primary-400);

      .value,
      .name {
        color: var(--primary-layer-0-color);
      }
    }

    &:active:not(.main-menu-opened) {
      opacity: 0.75;
      transform: scale(0.99);

      .bubbles {
        .left {
          &.small {
            transform: translateX(-0.5rem);
          }

          &.medium {
            transform: translateX(-0.25rem);
          }
        }

        .right {
          &.small {
            transform: translateX(0.5rem);
          }

          &.medium {
            transform: translateX(0.25rem);
          }
        }
      }
    }
  }

  @include medium-screen {
    width: calc(16rem - 1rem * 2);
  }

  :deep(button) {
    width: fit-content;
    height: 1.75rem;
    transition: var(--fast-transition-duration);
  }

  :deep(input) {
    border-color: #ffffff66;
    font-size: var(--small-font-size);
    color: var(--primary-layer-0-color);

    &:focus {
      border-color: #ffffff;
    }

    &::placeholder {
      color: var(--primary-layer-0-color);
    }
  }

  &.disabled {
    border-color: transparent;
    background: var(--primary-600);
    cursor: auto;
  }

  &.controller:not(.attention) {
    &:hover {
      border-color: #1d313640;
    }

    &:active {
      border-color: #1d313680;
    }
  }

  &.attention {
    border-color: transparent;
    background: var(--primary-300);

    .value,
    .name {
      opacity: 1;
      color: var(--white-original);
    }
  }

  &.controller.edit-menu-opened {
    height: calc(6.5rem - 2rem * 2);
    margin: 3rem 0 3rem 0;

    .bubbles {
      transform: scale(0.75) translate(-56%, 0.625rem);

      @include medium-screen {
        transform: scale(0.75) translate(-56%, 0.875rem);
      }
    }
  }

  &.main-menu-opened {
    border-color: transparent;
    box-shadow: var(--large-shadow);
    transform: scale(1.1);
    background: var(--primary-400);
    cursor: auto;

    @include medium-screen {
      transform: scale(1.125);
    }

    .bubbles {
      .left {
        &.small {
          transform: translateX(5.5rem);
        }

        &.medium {
          transform: translateX(3.25rem);
        }
      }

      .right {
        &.small {
          transform: translateX(-5.5rem);
        }

        &.medium {
          transform: translateX(-3.25rem);
        }
      }

      .big {
        transform: scale(1.125);
      }
    }

    .value,
    .button {
      opacity: 0;
      transform: translateY(-1rem);
      pointer-events: none;
      color: var(--primary-layer-0-color);
    }

    .name {
      opacity: 1;
      transform: translateY(-3rem);
      font-size: var(--medium-font-size);
      font-weight: 600;
      color: var(--primary-layer-0-color);
    }
  }

  &.edit-menu-opened {
    .bubbles {
      transform: scale(0.75) translate(-56%, -2.25rem);

      @include medium-screen {
        transform: scale(0.75) translate(-56%, -2.125rem);
      }
    }

    .name {
      opacity: 0;
      transform: translateY(-3.5rem);
    }
  }

  .bubbles {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
    width: 100%;
    transition: var(--fast-transition-duration);

    div {
      border-radius: 100%;
      background: v-bind(bubblesColor);
      font-size: initial;
      transition: var(--fast-transition-duration);
    }

    .small {
      width: 1rem;
      height: 1rem;
      opacity: 0.75;
    }

    .medium {
      width: 1.5rem;
      height: 1.5rem;
      opacity: 0.75;
    }

    .big {
      display: flex;
      justify-content: center;
      align-items: center;
      z-index: 1;
      width: 3.25rem;
      height: 3.25em;

      svg {
        width: 1.5rem;

        @include medium-screen {
          width: 1.75rem;
        }
      }
    }
  }

  .value {
    font-size: var(--medium-font-size);
    font-weight: 600;
    transition: var(--fast-transition-duration);
  }

  .name {
    opacity: 0.75;
    white-space: nowrap;
    font-size: var(--default-font-size);
    text-overflow: ellipsis;
    transition: var(--fast-transition-duration);
  }
}

body[data-theme='dark'] .card:not(.main-menu-opened):not(.disabled) {
  border-color: #ffffff66;

  &:not(.controller) {
    &:hover,
    &:active {
      border-color: transparent;
    }
  }

  &.controller {
    &:hover {
      border-color: #ffffff;
    }

    &:active {
      border-color: #ffffffbf;
    }
  }

  &.attention {
    border-color: transparent;
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "buttons": {
      "enable": "Enable",
      "start": "Start",
      "stop": "Stop",
      "open": "Open",
      "close": "Close"
    }
  },
  "ru-RU": {
    "buttons": {
      "enable": "Включить",
      "start": "Запустить",
      "stop": "Остановить",
      "open": "Открыть",
      "close": "Закрыть"
    }
  }
}
</i18n>
