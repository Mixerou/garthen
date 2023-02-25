<script setup>
import dateFormat, { i18n } from 'dateformat'

const props = defineProps({
  isEditLayoutVisible: {
    type: Boolean,
    required: false,
    default: false,
  },
  disabled: {
    type: Boolean,
    required: false,
    default: false,
  },
  email: {
    type: String,
    required: true,
  },
  username: {
    type: String,
    required: true,
  },
  newPassword: {
    type: String,
    required: true,
  },
  confirmedNewPassword: {
    type: String,
    required: true,
  },
  currentPassword: {
    type: String,
    required: true,
  },
  theme: {
    type: String,
    required: true,
  },
  locale: {
    type: String,
    required: true,
  },
})
const emit = defineEmits([
  'update:email',
  'update:username',
  'update:newPassword',
  'update:confirmedNewPassword',
  'update:currentPassword',
  'update:theme',
  'update:locale',
])

const { t, locales } = useI18n()
const constants = useConstantsStore()
const user = useUserStore()

const computedLocales = computed({
  get() {
    let preparedLocales = []

    locales.value.map(locale =>
      preparedLocales.push({ id: locale.code, value: locale.name })
    )

    return preparedLocales
  },
})

const computedEmail = computed({
  get() {
    return props.email
  },
  set(value) {
    if (props.disabled) return
    emit('update:email', value)
  },
})
const computedUsername = computed({
  get() {
    return props.username
  },
  set(value) {
    if (props.disabled) return
    emit('update:username', value)
  },
})
const computedNewPassword = computed({
  get() {
    return props.newPassword
  },
  set(value) {
    if (props.disabled) return
    emit('update:newPassword', value)
  },
})
const computedConfirmedNewPassword = computed({
  get() {
    return props.confirmedNewPassword
  },
  set(value) {
    if (props.disabled) return
    emit('update:confirmedNewPassword', value)
  },
})
const computedCurrentPassword = computed({
  get() {
    return props.currentPassword
  },
  set(value) {
    if (props.disabled) return
    emit('update:currentPassword', value)
  },
})
const computedTheme = computed({
  get() {
    return props.theme
  },
  set(value) {
    if (props.disabled) return
    emit('update:theme', value)
  },
})
const computedLocale = computed({
  get() {
    return props.locale
  },
  set(value) {
    if (props.disabled) return
    emit('update:locale', value)
  },
})

const createdAt = computed(() => {
  const date = new Date(user.createdAt * 1e3)

  i18n.monthNames = [
    t('monthShortNames.january'),
    t('monthShortNames.february'),
    t('monthShortNames.march'),
    t('monthShortNames.april'),
    t('monthShortNames.may'),
    t('monthShortNames.june'),
    t('monthShortNames.july'),
    t('monthShortNames.august'),
    t('monthShortNames.september'),
    t('monthShortNames.october'),
    t('monthShortNames.november'),
    t('monthShortNames.december'),
  ]

  return dateFormat(date, 'mmm dd, yyyy')
})
</script>

<template>
  <div
    class="middle-content"
    :class="{
      ['show-edit-container']: isEditLayoutVisible,
      ['show-current-password-group']:
        computedEmail !== user.email ||
        computedUsername !== user.username ||
        computedNewPassword !== '',
      ['show-new-password-confirmation-group']: computedNewPassword !== '',
      ['show-information-container']: !isEditLayoutVisible,
    }"
  >
    <Transition enter-from-class="hide" leave-to-class="hide" mode="out-in">
      <div v-if="isEditLayoutVisible" class="edit-container">
        <TransitionGroup
          enter-from-class="hide"
          enter-active-class="animate"
          leave-to-class="hide"
          leave-active-class="animate"
        >
          <div key="email-group" class="group">
            <div class="key">{{ $t('email') }}</div>
            <GarthenInput
              v-model:text="computedEmail"
              :disabled="disabled"
              :placeholder="user.email"
            />
          </div>
          <div key="username-group" class="group">
            <div class="key">{{ $t('username') }}</div>
            <GarthenInput
              v-model:text="computedUsername"
              :disabled="disabled"
              :placeholder="user.username"
            />
          </div>
          <div key="new-password-group" class="group">
            <div class="key">{{ t('editContainer.newPasswordKey') }}</div>
            <GarthenInput
              v-model:text="computedNewPassword"
              :disabled="disabled"
              type="password"
            />
          </div>
          <div
            v-if="computedNewPassword !== ''"
            key="new-password-confirmation-group"
            class="group"
          >
            <div class="key">
              {{ t('editContainer.newPasswordConfirmationKey') }}
            </div>
            <GarthenInput
              v-model:text="computedConfirmedNewPassword"
              :disabled="disabled"
              type="password"
            />
          </div>
          <div
            v-if="
              computedEmail !== user.email ||
              computedUsername !== user.username ||
              computedNewPassword !== ''
            "
            key="current-password-group"
            class="group"
          >
            <div class="key">{{ t('editContainer.currentPasswordKey') }}</div>
            <GarthenInput
              v-model:text="computedCurrentPassword"
              :disabled="disabled"
              type="password"
            />
          </div>
          <div key="theme-group" class="group row">
            <div class="key">{{ t('editContainer.themeKey') }}</div>
            <GarthenSelect
              v-model:selected="computedTheme"
              :disabled="disabled"
              :items="[
                { id: constants.USER_THEMES.light, value: $t('themes.light') },
                { id: constants.USER_THEMES.dark, value: $t('themes.dark') },
                { id: constants.USER_THEMES.auto, value: $t('themes.auto') },
              ]"
            />
          </div>
          <div key="language-group" class="group row">
            <div class="key">{{ t('editContainer.localeKey') }}</div>
            <GarthenSelect
              v-model:selected="computedLocale"
              :disabled="disabled"
              :items="computedLocales"
            />
          </div>
        </TransitionGroup>
      </div>

      <div v-else class="information-container">
        <div class="group">
          <div class="key">
            {{ t('informationContainer.groups.greenhousesKey') }}
          </div>
          <div class="value greenhouses-count">
            <Transition
              enter-from-class="move-to-bottom"
              leave-to-class="move-to-top"
            >
              <span :key="`greenhouses-count-${user.greenhousesCount}`">
                {{ user.greenhousesCount }}
              </span>
            </Transition>
          </div>
        </div>
        <div class="group">
          <div class="key">
            {{ t('informationContainer.groups.inProjectSinceKey') }}
          </div>
          <div class="value">{{ createdAt }}</div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<style lang="scss" scoped>
.middle-content {
  border-radius: var(--large-radius);
  color: var(--primary-layer-1-color);
  transition: width var(--default-transition), height var(--default-transition),
    margin-top var(--default-transition), padding-top var(--default-transition),
    padding-bottom var(--default-transition),
    padding-left var(--default-transition),
    padding-right var(--default-transition),
    opacity var(--fast-transition-duration),
    transform var(--fast-transition-duration),
    background-color var(--default-transition);

  &.show-edit-container {
    width: 100%;
    height: 21.8125rem;
    margin-top: calc(-4rem - 1rem);

    @include medium-screen {
      height: 23.625rem;
    }

    &.show-current-password-group {
      height: 27.25rem;

      @include medium-screen {
        height: 29.5rem;
      }

      &.show-new-password-confirmation-group {
        height: 32.6875rem;

        @include medium-screen {
          height: 35.375rem;
        }
      }
    }
  }

  &.show-information-container {
    width: calc(100% - 1rem * 2);
    height: 2.875rem;
    padding: 0.5rem 1rem;
    background: var(--primary-layer-1-background);
    cursor: pointer;

    &:hover {
      opacity: 0.9;
    }

    &:active {
      opacity: 0.75;
      transform: scale(0.99);
    }

    @include medium-screen {
      height: 3.25rem;
    }
  }

  .hide {
    opacity: 0;
  }

  .edit-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    transition: opacity var(--fast-transition-duration);

    .group {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      &.row {
        flex-direction: row;
        justify-content: space-between;
        align-items: center;

        div,
        select {
          width: 100%;
        }

        select {
          text-align: center;
        }
      }

      &.hide {
        margin-bottom: calc(-4.4375rem - 1rem);
        opacity: 0;

        @include medium-screen {
          margin-bottom: calc(-4.875rem - 1rem);
        }
      }

      &.animate {
        z-index: 999;
        transition: var(--default-transition);
      }

      .key {
        font-size: var(--default-font-size);
        font-weight: 600;
      }
    }
  }

  .edit-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    transition: opacity var(--fast-transition-duration);

    .group {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
    }
  }

  .information-container {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    transition: opacity var(--fast-transition-duration);

    .group {
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 0.25rem;

      .key {
        font-weight: 600;
      }

      .greenhouses-count {
        position: relative;
        width: 1rem;
        height: 1rem;

        span {
          position: absolute;
          top: 0;
          right: 0;
          transition-duration: var(--fast-transition-duration);

          &.move-to-top {
            opacity: 0;
            transform: translateY(-0.5rem);
          }

          &.move-to-bottom {
            opacity: 0;
            transform: translateY(0.5rem);
          }
        }
      }
    }
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "editContainer": {
      "newPasswordKey": "New password",
      "newPasswordConfirmationKey": "Confirm new password",
      "currentPasswordKey": "Current password",
      "themeKey": "Theme",
      "localeKey": "Language"
    },
    "informationContainer": {
      "groups": {
        "greenhousesKey": "Greenhouses",
        "inProjectSinceKey": "In project since"
      }
    }
  },
  "ru-RU": {
    "editContainer": {
      "newPasswordKey": "Новый пароль",
      "newPasswordConfirmationKey": "Подтвердите новый пароль",
      "currentPasswordKey": "Текущий пароль",
      "themeKey": "Тема",
      "localeKey": "Язык"
    },
    "informationContainer": {
      "groups": {
        "greenhousesKey": "Теплиц",
        "inProjectSinceKey": "В проекте с"
      }
    }
  }
}
</i18n>
