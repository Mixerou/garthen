<script setup>
import ImageFlowersLight from '@/assets/images/flowers-light.svg?skipsvgo'
import ImageFlowersDark from '@/assets/images/flowers-dark.svg?skipsvgo'

const { t, locale } = useI18n()
const user = useUserStore()

const isAuthModalOpened = ref(false)
</script>

<template>
  <section>
    <ProfileAuthModal
      v-if="isAuthModalOpened"
      @close="isAuthModalOpened = false"
    />

    <div class="information" :class="locale.toLowerCase().split('-')[0]">
      <h1>
        <span class="first-part">
          <span
            v-for="word in t('heading.firstPart').split(' ')"
            :key="`heading-word-${word}`"
          >
            <span v-if="word.toLowerCase() === 'garthen'" class="company-name">
              {{ `${word} ` }}
            </span>
            <span v-else>{{ `${word} ` }}</span>
          </span>
        </span>
        <br />
        <span class="second-part">{{ t('heading.secondPart') }}</span>
      </h1>
      <!--      TODO: Text-->
      <p>
        Elementum orci amet vulputate risus nisl dictum donec nibh. Egestas leo
        ante nunc eleifend tincidunt. Nisi at amet nam a interdum felis. Aliquet
        praesent a enim eget erat iaculis urna lacinia.
      </p>
      <NuxtLink v-if="user.isLoggedIn" class="action" to="/greenhouses">
        <GarthenButton>{{ $t('getStarted') }}</GarthenButton>
      </NuxtLink>
      <GarthenButton v-else class="action" @click="isAuthModalOpened = true">
        {{ $t('getStarted') }}
      </GarthenButton>
    </div>

    <div class="image">
      <ImageFlowersLight class="light" />
      <ImageFlowersDark class="dark" />
    </div>
  </section>
</template>

<style lang="scss" scoped>
section {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 2rem;
  width: 100%;
  height: calc(100vh - 6rem);
  min-height: 40rem;

  @include xl-screen {
    justify-content: space-between;
  }

  .information {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.25rem;
    width: 100%;
    max-width: 40rem;
    text-align: center;

    &.ru {
      max-width: 52rem;
    }

    @include medium-screen {
      gap: 1.5rem;
    }

    @include large-screen {
      gap: 1.75rem;
    }

    @include xl-screen {
      align-items: flex-start;
      gap: 2rem;
      text-align: start;
    }

    h1 {
      font-weight: 900;

      .first-part .company-name {
        color: var(--primary);
      }
    }

    p {
      line-height: var(--medium-line-height);
      font-size: var(--medium-font-size);
    }

    .action {
      width: fit-content;
      text-decoration: none;
    }
  }

  .image {
    display: none;

    @include xl-screen {
      display: flex;
      justify-content: center;
      align-items: center;
      width: 50%;
    }

    svg {
      width: 4rem;

      @include xl-screen {
        width: 32rem;
      }

      @include xxl-screen {
        width: 44rem;
      }
    }
  }
}

svg.dark {
  display: none;
}

body[data-theme='dark'] {
  svg.light {
    display: none;
  }

  svg.dark {
    display: initial;
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "heading": {
      "firstPart": "Welcome to Garthen",
      "secondPart": "Greenhouse Tools"
    }
  },
  "ru-RU": {
    "heading": {
      "firstPart": "Приветствуем в Garthen",
      "secondPart": "Инструментах для теплиц"
    }
  }
}
</i18n>
