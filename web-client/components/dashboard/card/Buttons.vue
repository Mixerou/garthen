<script setup>
defineProps({
  mainMenuOpened: {
    type: Boolean,
    required: true,
  },
  editMenuOpened: {
    type: Boolean,
    required: true,
  },
})
const emit = defineEmits(['open:edit-menu', 'close:edit-menu', 'disable'])

const { t } = useI18n()
</script>

<template>
  <div class="menu">
    <TransitionGroup enter-from-class="hide" leave-to-class="hide">
      <GarthenButton
        v-if="editMenuOpened"
        class="save"
        @click="emit('close:edit-menu')"
      >
        {{ t('buttons.back') }}
      </GarthenButton>
      <GarthenButton
        v-if="mainMenuOpened && !editMenuOpened"
        class="edit"
        @click="emit('open:edit-menu')"
      >
        {{ t('buttons.edit') }}
      </GarthenButton>
      <GarthenButton
        v-if="mainMenuOpened && !editMenuOpened"
        class="disable"
        variant="danger"
        @click="emit('disable')"
      >
        {{ t('buttons.disable') }}
      </GarthenButton>
    </TransitionGroup>
  </div>
</template>

<style lang="scss" scoped>
.menu {
  position: absolute;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  bottom: 0.5rem;

  .save {
    position: absolute;
    bottom: 0;
    transform: translateY(-1remrem);
  }

  .save,
  .edit {
    background: var(--primary-500);
  }

  .hide {
    opacity: 0;

    &.save {
      transform: translateY(-2rem);
    }

    &.edit {
      transform: translateY(1.5rem);
    }

    &.disable {
      transform: translateY(1rem);
    }
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "buttons": {
      "back": "Back",
      "disable": "Disable",
      "edit": "Edit"
    }
  },
  "ru-RU": {
    "buttons": {
      "back": "Назад",
      "disable": "Отключить",
      "edit": "Изменить"
    }
  }
}
</i18n>
