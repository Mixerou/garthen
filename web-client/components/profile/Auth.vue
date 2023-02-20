<script setup>
const { t, locale } = useI18n()
const { $authorizedFetch, $wsOpenConnection, $wsSubscribe } = useNuxtApp()
const constants = useConstantsStore()
const system = useSystemStore()
const user = useUserStore()

const isRegistrationTemplate = ref(false)
const isLoading = ref(false)
const isError = ref(false)

const email = ref('')
const username = ref('')
const password = ref('')
const confirmedPassword = ref('')

const isEmailIncorrect = ref(false)
const isUsernameIncorrect = ref(false)
const isPasswordIncorrect = ref(false)
const isConfirmedPasswordIncorrect = ref(false)

const error = ref('')

const switchTemplate = () => {
  isError.value = false
  isRegistrationTemplate.value = !isRegistrationTemplate.value
}

const register = async () => {
  let body = {
    email: email.value,
    password: password.value,
    username: username.value,
  }

  const response = await $authorizedFetch('/auth/register', {
    method: 'POST',
    body: JSON.stringify(body),
  })

  if (!response.ok) {
    const json = await response.json()

    isError.value = true
    error.value = constants.parseGlobalApiErrorCode(json.code)

    return
  }

  isError.value = false
}

const login = async () => {
  let body = {
    email: email.value,
    password: password.value,
  }

  const response = await $authorizedFetch('/auth/login', {
    method: 'POST',
    body: JSON.stringify(body),
  })

  if (!response.ok) {
    const json = await response.json()

    isError.value = true

    if (response.status === 404) {
      isEmailIncorrect.value = true
      isPasswordIncorrect.value = true
      error.value = ''

      return
    }

    error.value = constants.parseGlobalApiErrorCode(json.code)

    return
  }

  isError.value = false
}

const auth = async () => {
  isEmailIncorrect.value = false
  isUsernameIncorrect.value = false
  isPasswordIncorrect.value = false
  isConfirmedPasswordIncorrect.value = false

  // TODO: Make better validation
  setTimeout(async () => {
    if (email.value < 3 || !email.value.includes('@')) {
      isEmailIncorrect.value = true
    }

    if (password.value.length < 8) isPasswordIncorrect.value = true

    if (isRegistrationTemplate.value) {
      if (username.value.length < 3) isUsernameIncorrect.value = true

      if (
        confirmedPassword.value !== password.value ||
        confirmedPassword.value === ''
      )
        isConfirmedPasswordIncorrect.value = true
    }

    if (isEmailIncorrect.value || isPasswordIncorrect.value) return
    if (
      isRegistrationTemplate.value &&
      (isUsernameIncorrect.value || isConfirmedPasswordIncorrect.value)
    )
      return

    isLoading.value = true

    await (isRegistrationTemplate.value ? register() : login())

    let timeoutTime =
      error.value === ''
        ? 0
        : getComputedStyle(document.body)
            .getPropertyValue('--default-transition-duration')
            .split('s')[0] * 1000

    if (isError.value) {
      isLoading.value = false

      return
    }

    setTimeout(() => {
      isLoading.value = false

      user.setIsLoggedIn(true)
      $wsOpenConnection()
      $wsSubscribe('user/me', {}, true)

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
        <span :key="`error-${error}`">{{
          $t(`globalApiErrors.${error}`)
        }}</span>
      </Transition>
    </div>
    <div class="content">
      <transition enter-from-class="hide" leave-to-class="hide" mode="out-in">
        <h5 v-if="isRegistrationTemplate">{{ t('signUp', 2) }}</h5>
        <h5 v-else>{{ $t('signIn', 2) }}</h5>
      </transition>

      <div class="credentials">
        <TransitionGroup enter-from-class="hide" leave-to-class="hide">
          <GarthenInput
            key="email-field"
            v-model:text="email"
            type="email"
            :disabled="isLoading"
            :error="isEmailIncorrect"
            :placeholder="t('inputs.email')"
          />
          <GarthenInput
            v-if="isRegistrationTemplate"
            key="username-field"
            v-model:text="username"
            :disabled="isLoading"
            :error="isUsernameIncorrect"
            :placeholder="t('inputs.username')"
          />
          <GarthenInput
            key="password-field"
            v-model:text="password"
            type="password"
            :disabled="isLoading"
            :error="isPasswordIncorrect"
            :placeholder="t('inputs.password')"
          />
          <GarthenInput
            v-if="isRegistrationTemplate"
            key="password-confirmation-field"
            v-model:text="confirmedPassword"
            type="password"
            :disabled="isLoading"
            :error="isConfirmedPasswordIncorrect"
            :placeholder="t('inputs.passwordConfirmation')"
          />
        </TransitionGroup>
      </div>

      <div class="buttons">
        <GarthenButton :disabled="isLoading" @click="switchTemplate">
          <Transition
            enter-from-class="hide"
            leave-to-class="hide"
            mode="out-in"
          >
            <span v-if="isRegistrationTemplate">{{ $t('signIn') }}</span>
            <!--      TODO: Change the template so that t('signUp', 1) is displayed well in Russian -->
            <span v-else>{{ t('signUp', 2) }}</span>
          </Transition>
        </GarthenButton>

        <GarthenButton :loading="isLoading" @click="auth">
          <Transition
            enter-from-class="hide"
            leave-to-class="hide"
            mode="out-in"
          >
            <span v-if="locale !== 'ru-RU'">{{ $t('go') }}</span>
            <span v-else-if="isRegistrationTemplate">{{ $t('go') }}</span>
            <span v-else>{{ $t('signIn') }}</span>
          </Transition>
        </GarthenButton>
      </div>
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

    .hide {
      margin-bottom: -3.25rem;
      opacity: 0;

      @include medium-screen {
        margin-bottom: -3.5rem;
      }
    }
  }

  .buttons {
    display: flex;
    gap: 0.5rem;

    button span {
      transition-duration: var(--fast-transition-duration);
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
    "signUp": "Register",
    "inputs": {
      "email": "Email",
      "username": "Username",
      "password": "Password",
      "passwordConfirmation": "Confirm password"
    }
  },
  "ru-RU": {
    "signUp": "Зарегистрироваться | Регистрация",
    "inputs": {
      "email": "Адрес эл. почты",
      "username": "Имя пользователя",
      "password": "Пароль",
      "passwordConfirmation": "Подтвердить пароль"
    }
  }
}
</i18n>
