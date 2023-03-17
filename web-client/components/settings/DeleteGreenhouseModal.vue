<script setup>
const emit = defineEmits(['close'])

const { $wsSendAndWait } = useNuxtApp()
const { t } = useI18n()
const route = useRoute()
const constants = useConstantsStore()
const dataStore = useDataStore()
const system = useSystemStore()

const currentPassword = ref('')

const isDeleting = ref(false)
const isError = ref(false)
const error = ref('')

const deleteGreenhouse = async () => {
  isDeleting.value = true

  const response = await $wsSendAndWait({
    o: 2,
    r: 'greenhouse',
    m: { delete: null },
    d: {
      a: 'request_delete_greenhouse',
      id: BigInt(route.params.greenhouseId || 0),
      ['current_password']: currentPassword.value,
    },
  })

  if (response.d.code !== 200) {
    isDeleting.value = false
    isError.value = true
    error.value = constants.parseGlobalWsErrorCode(response.d.code)

    return
  }

  let timeoutTime =
    error.value === ''
      ? 0
      : getComputedStyle(document.body)
          .getPropertyValue('--default-transition-duration')
          .split('s')[0] * 1000

  setTimeout(() => {
    isDeleting.value = false

    // TODO: Make it safe
    system.unregisterActiveModal()
  }, timeoutTime)
}
</script>

<template>
  <GarthenModal
    close-on-click-outside
    :is-dropdown-visible="isError"
    @close="emit('close')"
  >
    <template #dropdown>
      <Transition enter-from-class="hide" leave-to-class="hide" mode="out-in">
        <span :key="`error-${error}`">
          {{ $t(`globalWsErrors.${error}`) }}
        </span>
      </Transition>
    </template>

    <template #content>
      <h5>{{ t('heading') }}</h5>

      <div class="credentials">
        <GarthenInput
          key="greenhouse-name-field"
          v-model:text="currentPassword"
          type="password"
          :disabled="isDeleting"
          :placeholder="$t('currentPassword')"
        />
      </div>

      <GarthenButton :disabled="isDeleting" @click="deleteGreenhouse">
        {{ $t('go') }}
      </GarthenButton>
    </template>
  </GarthenModal>
</template>

<style lang="scss" scoped>
.dropdown {
  span {
    transition-duration: var(--fast-transition-duration);

    &.hide {
      opacity: 0;
    }
  }
}

.content {
  .credentials {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
  }

  .hide {
    opacity: 0;
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "heading": "Disconnection"
  },
  "ru-RU": {
    "heading": "Отключение"
  }
}
</i18n>
