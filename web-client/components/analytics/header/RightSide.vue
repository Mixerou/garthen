<script setup>
import IconShowChart from '@/assets/icons/show-chart.svg?skipsvgo'
import IconTableRows from '@/assets/icons/table-rows.svg?skipsvgo'

defineProps({
  layout: {
    type: String,
    required: true,
    validator(layout) {
      return ['charts', 'tables'].includes(layout)
    },
  },
})
const emit = defineEmits(['update:layout'])

const { t } = useI18n()
</script>

<template>
  <div class="side">
    <GreenhousesSelector class="selector" />
    <div class="layout-selector">
      <div
        class="variant"
        :class="{ selected: layout === 'charts' }"
        @click="emit('update:layout', 'charts')"
      >
        <IconShowChart />
        <span>{{ t('layouts.charts') }}</span>
      </div>
      <div
        class="variant"
        :class="{ selected: layout === 'tables' }"
        @click="emit('update:layout', 'tables')"
      >
        <IconTableRows />
        <span>{{ t('layouts.tables') }}</span>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.side {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 0.5rem;

  .selector {
    margin-top: 0.25rem;
  }

  .layout-selector {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    height: calc(2.5rem - 0.25rem * 2);
    padding: 0.25rem 0.5rem;
    border-radius: var(--large-radius);
    background: var(--primary-layer-0-background);
    color: var(--primary-layer-1-background);

    @include medium-screen {
      gap: 0.75rem;
      height: calc(2.75rem - 0.5rem * 2);
      padding: 0.5rem 1rem;
    }

    .variant {
      display: flex;
      justify-content: center;
      align-items: center;
      padding: 0.25rem 0.5rem;
      border-radius: var(--medium-radius);
      color: var(--primary-layer-1-color);
      fill: var(--primary-layer-1-color);
      cursor: pointer;
      font-weight: 600;
      transition: var(--fast-transition-duration);

      &:hover {
        opacity: 0.9;
      }

      &:active {
        opacity: 0.5;
      }

      &.selected {
        background: var(--primary-layer-1-background);
      }

      svg {
        width: 1rem;
      }

      span {
        display: none;
      }

      @include large-screen {
        svg {
          display: none;
        }

        span {
          display: initial;
        }
      }
    }
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "layouts": {
      "charts": "Charts",
      "tables": "Tables"
    }
  },
  "ru-RU": {
    "layouts": {
      "charts": "Графики",
      "tables": "Таблицы"
    }
  }
}
</i18n>
