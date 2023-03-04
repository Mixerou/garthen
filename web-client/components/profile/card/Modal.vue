<script setup>
import IconLogout from '@/assets/icons/logout.svg?skipsvgo'
import IconArrowForward from '@/assets/icons/arrow-forward.svg?skipsvgo'

const emit = defineEmits(['close'])

const { t } = useI18n()
const { $authorizedFetch, $wsSendAndWait } = useNuxtApp()
const router = useRouter()
const constants = useConstantsStore()
const system = useSystemStore()
const user = useUserStore()

const isError = ref(false)
const error = ref('')

const editedEmail = ref('')
const editedUsername = ref('')
const newPassword = ref('')
const confirmedNewPassword = ref('')
const currentPassword = ref('')
const editedTheme = ref(0)
const editedLocale = ref('')

const isConfirmedNewPasswordIncorrect = ref(false)

const isOpenAppButtonVisible = ref(false)
const isEditLayoutVisible = ref(false)
const isLogoutLoading = ref(false)
const isEditsSaving = ref(false)

const switchLayout = () => {
  isEditLayoutVisible.value = !isEditLayoutVisible.value
  editedEmail.value = user.email
  editedUsername.value = user.username
  newPassword.value = ''
  confirmedNewPassword.value = ''
  currentPassword.value = ''
  editedLocale.value = user.locale
  editedTheme.value = user.theme
}

const save = () => {
  isConfirmedNewPasswordIncorrect.value = false

  // TODO: Make better validation
  setTimeout(async () => {
    if (
      newPassword.value !== '' &&
      confirmedNewPassword.value !== newPassword.value
    ) {
      isConfirmedNewPasswordIncorrect.value = true

      return
    }

    isEditsSaving.value = true

    const data = {
      a: 'request_patch_user',
      email: editedEmail.value,
      username: editedUsername.value,
      locale: editedLocale.value,
      theme: editedTheme.value,
      ['new_password']: newPassword.value,
      ['current_password']: currentPassword.value,
    }

    if (newPassword.value === '') delete data['new_password']
    if (currentPassword.value === '') delete data['current_password']

    const response = await $wsSendAndWait({
      o: 2,
      r: 'user/me',
      m: { patch: null },
      d: data,
    })

    isEditsSaving.value = false

    if (response.d.code !== 200) {
      isError.value = true
      error.value = constants.parseGlobalWsErrorCode(response.d.code)

      return
    }

    isError.value = false

    switchLayout()
  }, 1)
}

const logout = async () => {
  isLogoutLoading.value = true

  const response = await $authorizedFetch('/auth/logout', { method: 'POST' })

  isLogoutLoading.value = false

  // TODO: Make it safe
  system.unregisterActiveModal()

  if (response.ok) {
    const timeoutTime =
      getComputedStyle(document.body)
        .getPropertyValue('--default-transition-duration')
        .split('s')[0] * 1000

    setTimeout(() => user.logout(), timeoutTime)
  }
}

const onOpenAppButtonClick = () => {
  const timeoutTime =
    getComputedStyle(document.body)
      .getPropertyValue('--default-transition-duration')
      .split('s')[0] * 1000

  system.unregisterActiveModal()
  setTimeout(() => {
    router.push('/greenhouses')
  }, timeoutTime / 2)
}

onMounted(() => {
  if (!system.isAppLayout) {
    const timeoutTime =
      getComputedStyle(document.body)
        .getPropertyValue('--default-transition-duration')
        .split('s')[0] * 1000

    setTimeout(() => {
      isOpenAppButtonVisible.value = true
    }, timeoutTime / 2)
  }
})
</script>

<template>
  <GarthenModal
    close-on-click-outside
    :disable-dropdown-padding="!isError"
    :is-dropdown-visible="
      (isEditLayoutVisible && isError) ||
      (!isEditLayoutVisible && isOpenAppButtonVisible)
    "
    @close="emit('close')"
  >
    <template #dropdown>
      <Transition enter-from-class="hide" leave-to-class="hide" mode="out-in">
        <Transition
          v-if="isError"
          enter-from-class="hide"
          leave-to-class="hide"
          mode="out-in"
          class="dropdown-transition"
        >
          <span :key="`error-${error}`" class="dropdown-transition">
            {{ $t(`globalWsErrors.${error}`) }}
          </span>
        </Transition>
        <GarthenButton
          v-else
          class="dropdown-transition"
          @click="onOpenAppButtonClick"
        >
          <div class="custom-content">
            <span>{{ t('openAppButton') }}</span>
            <IconArrowForward class="icon" />
          </div>
        </GarthenButton>
      </Transition>
    </template>

    <template #content>
      <div class="heading" :class="{ hide: isEditLayoutVisible }">
        <ProfileAvatar class="avatar" :username="user.username" />
        <div class="main-information">
          <h5>{{ user.username }}</h5>
          <p>{{ user.email }}</p>
        </div>
      </div>

      <ProfileCardModalMiddleContent
        v-model:email="editedEmail"
        v-model:username="editedUsername"
        v-model:new-password="newPassword"
        v-model:confirmed-new-password="confirmedNewPassword"
        v-model:current-password="currentPassword"
        v-model:theme="editedTheme"
        v-model:locale="editedLocale"
        :is-edit-layout-visible="isEditLayoutVisible"
        :confirmed-new-password-incorrect="isConfirmedNewPasswordIncorrect"
        :disabled="isEditsSaving"
        @click="isEditLayoutVisible ? null : switchLayout()"
      />

      <Transition enter-from-class="hide" leave-to-class="hide" mode="out-in">
        <GarthenButton
          v-if="isEditLayoutVisible"
          :loading="isEditsSaving"
          @click="save"
        >
          <span>{{ t('saveButton') }}</span>
        </GarthenButton>
        <GarthenButton v-else :loading="isLogoutLoading" @click="logout">
          <IconLogout class="icon" />
          <span>{{ t('signOutButton') }}</span>
        </GarthenButton>
      </Transition>
    </template>
  </GarthenModal>
</template>

<style lang="scss" scoped>
.dropdown {
  button {
    width: calc(100% - 1.5rem * 2);

    &:active .custom-content .icon {
      transform: translateX(0.5rem);
    }

    .custom-content {
      display: flex;
      justify-content: space-between;
      align-items: center;
      width: 100%;

      .icon {
        transition: var(--fast-transition-duration);
      }
    }
  }

  .hide {
    opacity: 0;
  }

  .dropdown-transition {
    transition: var(--fast-transition-duration);
  }
}

.content {
  .heading {
    display: flex;
    align-items: center;
    gap: 1rem;
    width: 100%;
    transition: opacity var(--fast-transition-duration);
    transition-delay: var(--fast-transition-duration);

    &.hide {
      opacity: 0;
      pointer-events: none;
      transition-delay: 0s;
    }

    .avatar {
      width: 4rem;
      height: 4rem;
      border-radius: var(--large-radius);
    }

    .main-information {
      display: flex;
      flex-direction: column;
      align-items: center;
      width: calc(100% - 5rem);

      h5 {
        transition-duration: var(--fast-transition-duration);
      }

      p {
        max-width: 100%;
        opacity: 0.75;
        overflow: hidden;
        white-space: nowrap;
        font-weight: 600;
        text-overflow: ellipsis;
      }
    }
  }

  button {
    transition: all var(--fast-transition-duration);

    &.hide {
      opacity: 0;
    }
  }

  .icon {
    width: 1.5rem;

    @include medium-screen {
      width: 1.75rem;
    }
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "saveButton": "Save",
    "signOutButton": "Sign out",
    "openAppButton": "Open app"
  },
  "ru-RU": {
    "saveButton": "Сохранить",
    "signOutButton": "Выйти",
    "openAppButton": "Открыть приложение"
  }
}
</i18n>
