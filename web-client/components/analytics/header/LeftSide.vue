<script setup>
import IconCalendarMonth from '@/assets/icons/calendar-month.svg?skipsvgo'

const props = defineProps({
  range: {
    type: Number,
    required: true,
  },
  loading: {
    type: Boolean,
    required: false,
    default: false,
  },
})
const emit = defineEmits(['select:range'])

const { t } = useI18n()
const constants = useConstantsStore()

const selector = ref(null)

const computedSelectedRange = computed({
  get() {
    return props.range
  },
  set(value) {
    emit('select:range', value)
  },
})

const openSelector = () => {
  setTimeout(() => selector.value.firstChild.firstChild.click(), 10)
}
</script>

<template>
  <div class="left-side">
    <h3>{{ t('heading') }}</h3>

    <div class="timestamp-range" :class="{ loading }">
      <div class="content" @click="openSelector">
        <IconCalendarMonth />
        <p>{{ t(`ranges.${range}`) }}</p>
      </div>
      <div ref="selector">
        <GarthenSelect
          v-model:selected="computedSelectedRange"
          :disabled="loading"
          :items="[
            {
              id: constants.DEVICE_RECORDS_TIMESTAMP_RANGES.today,
              value: t(
                `ranges.${constants.DEVICE_RECORDS_TIMESTAMP_RANGES.today}`
              ),
            },
            {
              id: constants.DEVICE_RECORDS_TIMESTAMP_RANGES.week,
              value: t(
                `ranges.${constants.DEVICE_RECORDS_TIMESTAMP_RANGES.week}`
              ),
            },
            {
              id: constants.DEVICE_RECORDS_TIMESTAMP_RANGES.month,
              value: t(
                `ranges.${constants.DEVICE_RECORDS_TIMESTAMP_RANGES.month}`
              ),
            },
            {
              id: constants.DEVICE_RECORDS_TIMESTAMP_RANGES.lastMonth,
              value: t(
                `ranges.${constants.DEVICE_RECORDS_TIMESTAMP_RANGES.lastMonth}`
              ),
            },
            {
              id: constants.DEVICE_RECORDS_TIMESTAMP_RANGES.monthBeforeLast,
              value: t(
                `ranges.${constants.DEVICE_RECORDS_TIMESTAMP_RANGES.monthBeforeLast}`
              ),
            },
            {
              id: constants.DEVICE_RECORDS_TIMESTAMP_RANGES.lastThreeMoths,
              value: t(
                `ranges.${constants.DEVICE_RECORDS_TIMESTAMP_RANGES.lastThreeMoths}`
              ),
            },
          ]"
        />
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.left-side {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  .timestamp-range {
    position: relative;

    &:not(.loading) {
      .content:hover {
        svg,
        p {
          opacity: 0.9;
        }
      }

      .content:active {
        svg,
        p {
          opacity: 0.5;
        }
      }
    }

    &.loading .content {
      svg,
      p {
        opacity: 0.25;
        cursor: wait;
      }
    }

    .content {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      width: fit-content;
      opacity: 0.75;
      cursor: pointer;
    }

    svg {
      width: 1.25rem;
      fill: var(--layer-0-color);
      transition: var(--fast-transition-duration);

      @include medium-screen {
        width: 1.5rem;
      }
    }

    p {
      transition: opacity var(--fast-transition-duration);
    }

    .select {
      position: absolute;
      top: -0.5rem;
      left: 0;
      width: 100%;
      min-width: 12.875rem;
      cursor: auto;
      pointer-events: none;

      :deep(.selected-value) {
        opacity: 0;
      }

      :deep(.options) {
        pointer-events: initial;
      }

      @include medium-screen {
        min-width: 14.125rem;
      }
    }
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "heading": "Analytics",
    "ranges": {
      "0": "Today",
      "1": "This week",
      "2": "This month",
      "3": "Last month",
      "4": "Month before last",
      "5": "Last three months"
    }
  },
  "ru-RU": {
    "heading": "Аналитика",
    "ranges": {
      "0": "Сегодня",
      "1": "Эта неделя",
      "2": "Этот месяц",
      "3": "Прошлый месяц",
      "4": "Позапрошлый месяц",
      "5": "Последние три месяца"
    }
  }
}
</i18n>
