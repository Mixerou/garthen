<script setup>
import IconSave from '@/assets/icons/save.svg?skipsvgo'

defineProps({
  saving: {
    type: Boolean,
    required: false,
    default: false,
  },
})
const emit = defineEmits(['save'])

const { t } = useI18n()
</script>

<template>
  <div class="left-side">
    <h3>{{ t('heading') }}</h3>

    <div class="action" :class="{ saving }" @click="emit('save')">
      <IconSave />
      <p>{{ $t('save') }}</p>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.left-side {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  .action {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: fit-content;
    opacity: 0.75;
    cursor: pointer;
    transition: opacity var(--fast-transition-duration);

    &:not(.saving) {
      &:hover {
        opacity: 0.9;
      }

      &:active {
        opacity: 0.5;
      }
    }

    &.saving {
      opacity: 0.25;
      cursor: wait;
    }

    svg {
      width: 1.25rem;
      fill: var(--layer-0-color);
      transition: var(--fast-transition-duration);

      @include medium-screen {
        width: 1.5rem;
      }
    }
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "heading": "Settings"
  },
  "ru-RU": {
    "heading": "Настройки"
  }
}
</i18n>
