<script setup>
import dateFormat, { i18n } from 'dateformat'
import IconLogout from '@/assets/icons/logout.svg?skipsvgo'

const { t } = useI18n()
const { $authorizedFetch } = useNuxtApp()
const system = useSystemStore()
const user = useUserStore()

const isLogoutLoading = ref(false)

const logout = async () => {
  isLogoutLoading.value = true

  const response = await $authorizedFetch('/auth/logout', {
    method: 'POST',
  })

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

const createdAt = computed({
  get() {
    const date = new Date(user.createdAt * 1000)

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
  },
})
</script>

<template>
  <div class="container">
    <div class="heading">
      <ProfileAvatar class="avatar" :username="user.username" />
      <div class="main-information">
        <h5>{{ user.username }}</h5>
        <p>{{ user.email }}</p>
      </div>
    </div>

    <div class="information">
      <div class="group">
        <div class="key">{{ t('groups.greenhousesKey') }}</div>
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
        <div class="key">{{ t('groups.inProjectSinceKey') }}</div>
        <div class="value">{{ createdAt }}</div>
      </div>
    </div>

    <GarthenButton :loading="isLogoutLoading" @click="logout">
      <IconLogout class="icon" />
      <span>{{ t('signOutButton') }}</span>
    </GarthenButton>
  </div>
</template>

<style lang="scss" scoped>
.container {
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
  font-size: var(--default-font-size);
  color: var(--primary-layer-0-color);

  .heading {
    display: flex;
    align-items: center;
    gap: 1rem;
    width: 100%;

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
        max-width: 100%;
        overflow: hidden;
        white-space: nowrap;
        font-weight: 900;
        text-overflow: ellipsis;
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

  .information {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: calc(100% - 1rem * 2);
    padding: 0.5rem 1rem;
    border-radius: var(--large-radius);
    background: var(--primary-layer-1-background);
    color: var(--primary-layer-1-color);

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
    "signOutButton": "Sign out",
    "groups": {
      "greenhousesKey": "Greenhouses",
      "inProjectSinceKey": "In project since"
    }
  },
  "ru-RU": {
    "signOutButton": "Выйти",
    "groups": {
      "greenhousesKey": "Теплиц",
      "inProjectSinceKey": "В проекте с"
    }
  }
}
</i18n>
