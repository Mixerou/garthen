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
    required: false,
    default: 0,
  },
  name: {
    type: String,
    required: true,
  },
  kind: {
    type: Number,
    required: true,
  },
  value: {
    type: Number,
    required: false,
    default: null,
  },
})

const { t } = useI18n()
const { $wsSendAndWait } = useNuxtApp()
const route = useRoute()
const constants = useConstantsStore()

const isActionButtonLoading = ref(false)

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

watchEffect(() => {
  props.value

  isActionButtonLoading.value = false
})
</script>

<template>
  <div
    class="card"
    :class="{
      controller: isController,
      attention: isController && value === 1,
    }"
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
      v-if="isController && kind === constants.DEVICE_KINDS.windowsController"
      :loading="isActionButtonLoading"
      @click="changeState"
    >
      {{ computedValue === 1 ? t('buttons.close') : t('buttons.open') }}
    </GarthenButton>
    <GarthenButton
      v-else-if="isController"
      :loading="isActionButtonLoading"
      @click="changeState"
    >
      {{ computedValue === 1 ? t('buttons.stop') : t('buttons.start') }}
    </GarthenButton>
    <p v-else class="value">{{ computedValue }}</p>
    <p class="name">
      {{
        name === null
          ? t(`defaultNames.${kind}`, { externalId: externalId })
          : name
      }}
    </p>
  </div>
</template>

<style lang="scss" scoped>
.card {
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
  cursor: pointer;
  transition: var(--fast-transition-duration);

  &:not(.controller) {
    &:hover {
      border-color: transparent;
      background: var(--primary-400);
      color: var(--primary-layer-0-color);
    }

    &:active {
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

  &.attention {
    border-color: transparent;
    background: var(--primary-300);
    color: var(--white-original);
  }

  .bubbles {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
    width: 100%;

    div {
      border-radius: 100%;
      background: v-bind(bubblesColor);
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
  }

  .name {
    opacity: 0.75;
    font-size: var(--default-font-size);
  }
}

body[data-theme='dark'] .card {
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
    "defaultNames": {
      "0": "Humidity percentage {externalId}",
      "1": "Soil moisture {externalId}",
      "2": "Temperature {externalId}",
      "3": "Humidification controller",
      "4": "Irrigation controller {externalId}",
      "5": "Windows controller"
    },
    "buttons": {
      "start": "Start",
      "stop": "Stop",
      "open": "Open",
      "close": "Close"
    }
  },
  "ru-RU": {
    "defaultNames": {
      "0": "Процент влажности {externalId}",
      "1": "Влажность почвы {externalId}",
      "2": "Температура {externalId}",
      "3": "Контроллер влажности",
      "4": "Контроллер полива {externalId}",
      "5": "Контроллер форточек"
    },
    "buttons": {
      "start": "Запустить",
      "stop": "Остановить",
      "open": "Открыть",
      "close": "Закрыть"
    }
  }
}
</i18n>
