<script setup>
import IconLogout from '@/assets/icons/logout.svg?skipsvgo'

const { t } = useI18n()
const system = useSystemStore()
const user = useUserStore()

const logout = () => {
  const timeoutTime =
    getComputedStyle(document.body)
      .getPropertyValue('--default-transition-duration')
      .split('s')[0] * 1000

  // TODO: Make it safe
  system.unregisterActiveModal()
  setTimeout(() => user.logout(), timeoutTime)
}
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
        <div class="value">3</div>
      </div>
      <div class="group">
        <div class="key">{{ t('groups.inProjectSinceKey') }}</div>
        <div class="value">01/01/23</div>
      </div>
    </div>

    <GarthenButton @click="logout">
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
