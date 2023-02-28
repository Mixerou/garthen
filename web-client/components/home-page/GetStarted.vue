<script setup>
const { t } = useI18n()
const user = useUserStore()

const isAuthModalOpened = ref(false)
</script>

<template>
  <section>
    <ProfileAuthModal
      v-if="isAuthModalOpened"
      @close="isAuthModalOpened = false"
    />

    <h2>{{ t('heading') }}</h2>

    <NuxtLink v-if="user.isLoggedIn" class="action" to="/greenhouses">
      <GarthenButton>{{ $t('getStarted') }}</GarthenButton>
    </NuxtLink>
    <GarthenButton v-else class="action" @click="isAuthModalOpened = true">
      {{ $t('getStarted') }}
    </GarthenButton>
  </section>
</template>

<style lang="scss" scoped>
section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  padding: 2rem 0;

  @include medium-screen {
    padding: 4rem 0;
  }

  @include xxl-screen {
    padding: 6rem 0;
  }

  .action {
    text-decoration: none;
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "heading": "Connect to us!"
  },
  "ru-RU": {
    "heading": "Подключайся к нам!"
  }
}
</i18n>
