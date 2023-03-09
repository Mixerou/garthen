<script setup>
import IconGridView from '@/assets/icons/grid-view.svg?skipsvgo'
import IconBarChart from '@/assets/icons/bar-chart.svg?skipsvgo'

defineProps({
  fold: {
    type: Boolean,
    required: false,
    default: false,
  },
})

const { t } = useI18n()
const system = useSystemStore()

const isMobile = ref(true)
const screenChecker = ref(null)
const screenCheckerObserver = new ResizeObserver(event => {
  isMobile.value = screenChecker.value.clientWidth === 0
})

onMounted(() => {
  screenCheckerObserver.observe(screenChecker.value)
})

onBeforeUnmount(() => {
  screenCheckerObserver.unobserve(screenChecker.value)
})
</script>

<template>
  <nav :class="{ fold: fold, ['mobile']: isMobile }">
    <div ref="screenChecker" class="screen-checker" />
    <GarthenLogo :small="fold || isMobile" />
    <div class="buttons">
      <NuxtLink
        :class="{ selected: system.appPageName === 'dashboard' }"
        :to="`/greenhouses/${$route.params.greenhouseId}`"
      >
        <GarthenButton
          :transparent-background="system.appPageName !== 'dashboard'"
        >
          <IconGridView />
          <span>{{ t('buttons.dashboard') }}</span>
        </GarthenButton>
      </NuxtLink>
      <NuxtLink
        :class="{ selected: system.appPageName === 'analytics' }"
        :to="`/greenhouses/${$route.params.greenhouseId}/analytics`"
      >
        <GarthenButton
          :transparent-background="system.appPageName !== 'analytics'"
        >
          <IconBarChart />
          <span>{{ t('buttons.analytics') }}</span>
        </GarthenButton>
      </NuxtLink>
    </div>
    <Profile class="profile" :avatar-only="fold || isMobile" />
  </nav>
</template>

<style lang="scss" scoped>
nav {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4rem;
  width: 20rem;
  height: calc(100vh - 1.5rem);
  padding-top: 1.5rem;
  background: #e9f0ee;
  transition: var(--default-transition);
  transition-property: width, transform;

  @include medium-screen {
    height: calc(100vh - 2rem);
    padding: 2rem 2rem 0;
  }

  &.fold,
  &.mobile {
    width: 5.5rem;

    @include medium-screen {
      width: calc(7.75rem - 2rem * 2);
    }
  }

  &.fold .buttons {
    opacity: 0;
    pointer-events: none;

    button {
      width: calc(5.25rem - 2rem * 2);

      @include medium-screen {
        width: calc(5.25rem - 1.75rem * 2);
      }

      span {
        width: 0;
        margin-right: calc(-100% - 0.5rem);
        opacity: 0;
      }
    }
  }

  &.mobile .buttons button {
    padding: 0 1.25rem;

    svg {
      margin-left: 0.25rem;
    }

    span {
      width: 0;
      margin-right: calc(-100% - 0.5rem);
      opacity: 0;
    }
  }

  .screen-checker {
    position: absolute;
    width: 0;
    height: 0;
    background: transparent;
    pointer-events: none;

    @include medium-screen {
      width: 1rem;
      height: 1rem;
    }
  }

  .buttons {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    width: 100%;
    height: 100%;
    transition: opacity var(--default-transition);

    a {
      display: flex;
      justify-content: center;
      width: 100%;
      text-decoration: none;
      transition: opacity var(--fast-transition-duration);

      &:not(.selected) {
        opacity: 0.75;

        &:hover,
        &:active {
          opacity: 1;
        }
      }

      button {
        width: calc(100% - 3.5rem);

        svg {
          width: 1.5rem;

          @include medium-screen {
            width: 1.75rem;
          }
        }

        span {
          overflow: hidden;
          white-space: nowrap;
          text-overflow: ellipsis;
        }
      }
    }
  }

  .profile {
    margin-bottom: 4rem;

    @include medium-screen {
      margin-bottom: 1rem;
    }
  }
}

body[data-theme='dark'] nav {
  background: #1c2b31;
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "buttons": {
      "dashboard": "Dashboard",
      "analytics": "Analytics"
    }
  },
  "ru-RU": {
    "buttons": {
      "dashboard": "Панель управления",
      "analytics": "Аналитика"
    }
  }
}
</i18n>
