<script setup>
defineProps({
  disabled: {
    type: Boolean,
    required: false,
    default: false,
  },
})
const emit = defineEmits(['delete'])

const { t } = useI18n()
</script>

<template>
  <section>
    <h5>{{ t('heading') }}</h5>

    <div class="buttons">
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

<style lang="scss" scoped></style>

<i18n lang="json">
{
  "en-GB": {
    "heading": "Danger zone",
    "buttons": {
      "disconnect": "Disconnect greenhouse"
    }
  },
  "ru-RU": {
    "heading": "Опасная зона",
    "buttons": {
      "disconnect": "Отключить теплицу"
    }
  }
}
</i18n>
