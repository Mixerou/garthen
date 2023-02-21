<script setup>
const { $wsSendAndWait } = useNuxtApp()
const { t } = useI18n()
const constants = useConstantsStore()
const system = useSystemStore()

const isLoading = ref(false)
const isError = ref(false)
const error = ref('')

const greenhouseName = ref('')
const token = ref('')

const isGreenhouseNameIncorrect = ref(false)
const isTokenIncorrect = ref(false)

const create = () => {
  isGreenhouseNameIncorrect.value = false
  isTokenIncorrect.value = false

  // TODO: Make better validation
  setTimeout(async () => {
    if (greenhouseName.value.length < 3 || greenhouseName.value.length > 32) {
      isGreenhouseNameIncorrect.value = true
    }

    if (token.value.length < 3 || token.value.length > 32) {
      isTokenIncorrect.value = true
    }

    if (isGreenhouseNameIncorrect.value || isTokenIncorrect.value) return

    isLoading.value = true

    const response = await $wsSendAndWait({
      o: 2,
      r: 'greenhouse',
      m: { post: null },
      d: {
        name: greenhouseName.value,
        token: token.value,
      },
    })

    if (response.d.code !== 200) {
      isLoading.value = false
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
      isLoading.value = false

      // TODO: Make it safe
      system.unregisterActiveModal()
    }, timeoutTime)
  }, 1)
}
</script>

<template>
  <div class="container">
    <div class="error-container" :class="{ hide: !isError || error === '' }">
      <Transition enter-from-class="hide" leave-to-class="hide" mode="out-in">
        <span :key="`error-${error}`">
          {{ $t(`globalWsErrors.${error}`) }}
        </span>
      </Transition>
    </div>
    <div class="content">
      <h5>{{ t('connection') }}</h5>

      <div class="credentials">
        <GarthenInput
          key="greenhouse-name-field"
          v-model:text="greenhouseName"
          type="email"
          :disabled="isLoading"
          :error="isGreenhouseNameIncorrect"
          :placeholder="t('inputs.greenhouseName')"
        />
        <GarthenInput
          key="token-field"
          v-model:text="token"
          type="password"
          :disabled="isLoading"
          :error="isTokenIncorrect"
          :placeholder="t('inputs.token')"
        />
      </div>

      <GarthenButton :disabled="isLoading" @click="create">
        {{ $t('go') }}
      </GarthenButton>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.container {
  position: relative;

  .error-container {
    position: absolute;
    display: flex;
    flex-direction: column;
    align-items: center;
    bottom: 1rem;
    left: 0;
    width: 18rem;
    max-width: calc(100vw - 5rem);
    padding: 1rem 1.5rem;
    border-radius: var(--large-radius);
    box-shadow: var(--large-shadow);
    background: var(--primary);
    color: var(--primary-layer-0-color);
    transform: translateY(calc(100% + 1.5rem));
    transition: var(--default-transition);

    &.hide {
      box-shadow: unset;
      transform: translateY(0);
    }

    span {
      transition-duration: var(--fast-transition-duration);

      &.hide {
        opacity: 0;
      }
    }
  }
}

.content {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  width: 18rem;
  max-width: calc(100vw - 5rem);
  padding: 1rem 1.5rem;
  border-radius: var(--large-radius);
  box-shadow: var(--large-shadow);
  background: var(--primary);
  color: var(--primary-layer-0-color);

  h5 {
    font-weight: 900;
    transition-duration: var(--fast-transition-duration);
  }

  .credentials {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;

    input {
      border-color: #ffffff66;
      color: var(--primary-layer-0-color);

      &:focus {
        border-color: #ffffff;
      }

      &::placeholder {
        color: var(--primary-layer-0-color);
      }

      &.v-enter-active,
      &.v-leave-active {
        transition: var(--default-transition);
      }
    }
  }

  .hide {
    opacity: 0;
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "inputs": {
      "greenhouseName": "Greenhouse name",
      "token": "Token"
    },
    "connection": "Connection"
  },
  "ru-RU": {
    "inputs": {
      "greenhouseName": "Название теплицы",
      "token": "Токен"
    },
    "connection": "Подключение"
  }
}
</i18n>
