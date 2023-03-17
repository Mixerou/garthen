<script setup>
defineProps({
  disabled: {
    type: Boolean,
    required: false,
    default: false,
  },
})
const emit = defineEmits(['resetDeviceNames', 'delete'])

const { t } = useI18n()
</script>

<template>
  <section>
    <h5>{{ t('heading') }}</h5>

    <div class="buttons">
      <GarthenButton
        :disabled="disabled"
        variant="danger"
        @click="emit('resetDeviceNames')"
      >
        {{ t('buttons.resetDeviceNames') }}
      </GarthenButton>
      <GarthenButton
        :disabled="disabled"
        variant="danger"
        @click="emit('delete')"
      >
        {{ t('buttons.disconnect') }}
      </GarthenButton>
    </div>
  </section>
</template>

<style lang="scss" scoped>
.buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "heading": "Danger zone",
    "buttons": {
      "resetDeviceNames": "Reset device names",
      "disconnect": "Disconnect greenhouse"
    }
  },
  "ru-RU": {
    "heading": "Опасная зона",
    "buttons": {
      "resetDeviceNames": "Сбросить названия устройств",
      "disconnect": "Отключить теплицу"
    }
  }
}
</i18n>
