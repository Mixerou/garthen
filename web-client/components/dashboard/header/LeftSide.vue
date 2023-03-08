<script setup>
import IconRefresh from '@/assets/icons/refresh.svg?skipsvgo'

defineProps({
  refreshing: {
    type: Boolean,
    required: false,
    default: false,
  },
})
const emit = defineEmits(['refresh'])

const { t } = useI18n()
const user = useUserStore()
</script>

<template>
  <div class="left-side">
    <h3 class="with-greeting">
      <span
        v-for="(letter, index) in t('headingWithGreeting').split('')"
        :key="`heading-letter-${index}`"
        :class="{ username: letter === '#' }"
      >
        {{ letter === '#' ? user.username : letter }}
      </span>
    </h3>

    <h3 class="without-greeting">{{ t('heading') }}</h3>

    <div
      class="last-activity"
      :class="{ refreshing: refreshing }"
      @click="emit('refresh')"
    >
      <IconRefresh />
      <!-- TODO: Last activity time -->
      <p>{{ t('refreshData') }}</p>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.left-side {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  h3 {
    &.with-greeting {
      display: none;

      @include medium-screen {
        display: initial;
      }

      .username {
        font-weight: 900;
        color: var(--primary);
      }
    }

    &.without-greeting {
      display: initial;

      @include medium-screen {
        display: none;
      }
    }
  }

  .last-activity {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: fit-content;
    opacity: 0.75;
    cursor: pointer;
    transition: opacity var(--fast-transition-duration);

    &:not(.refreshing) {
      &:hover {
        opacity: 0.9;
      }

      &:active {
        opacity: 0.5;

        svg {
          transform: rotate(90deg);
        }
      }
    }

    &.refreshing {
      opacity: 0.25;
      cursor: wait;

      svg {
        animation: spin 1s;
      }
    }

    svg {
      width: 1.25rem;
      transform: rotate(45deg);
      fill: var(--layer-0-color);
      transition: var(--fast-transition-duration);

      @include medium-screen {
        width: 1.5rem;
      }
    }
  }
}

@keyframes spin {
  from {
    transform: rotate(90deg);
  }

  to {
    transform: rotate(405deg);
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "heading": "Dashboard",
    "headingWithGreeting": "Welcome back, #!",
    "refreshData": "Refresh data"
  },
  "ru-RU": {
    "heading": "Панель управления",
    "headingWithGreeting": "С возвращением, #!",
    "refreshData": "Обновить данные"
  }
}
</i18n>
