<script setup>
defineProps({
  deviceId: {
    type: BigInt,
    required: true,
  },
  controller: {
    type: Boolean,
    required: false,
    default: false,
  },
  editMenuOpened: {
    type: Boolean,
    required: true,
  },
  value: {
    type: String,
    required: true,
  },
})

const { t } = useI18n()
const dataStore = useDataStore()
</script>

<template>
  <div v-if="!controller" class="menu">
    <TransitionGroup enter-from-class="hide" leave-to-class="hide">
      <div v-if="editMenuOpened" class="container">
        <p class="heading">{{ t('current') }}</p>
        <p class="data">{{ value }}</p>
      </div>
      <div v-if="editMenuOpened" class="container">
        <p class="heading">{{ t('savedData') }}</p>
        <p class="data">
          {{ dataStore.deviceRecordsQuantities[deviceId]?.quantity }}
        </p>
      </div>
    </TransitionGroup>
  </div>
</template>

<style lang="scss" scoped>
.menu {
  position: absolute;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 0.5rem;
  top: 4.25rem;
  right: 1rem;
  width: calc(100% - 1rem * 2 - 1rem * 2);

  .container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    padding: 0.5rem 1rem;
    border-radius: var(--large-radius);
    background: var(--primary-layer-1-background);
    color: var(--primary-layer-1-color);
    transition: var(--fast-transition-duration);

    &.hide {
      opacity: 0;
      transform: translateY(1rem);
    }

    .heading {
      font-size: var(--default-font-size);
      font-weight: 600;
    }

    .data {
      font-size: var(--small-font-size);
    }
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "current": "Current",
    "savedData": "Saved data"
  },
  "ru-RU": {
    "current": "Сейчас",
    "savedData": "Всего данных"
  }
}
</i18n>
