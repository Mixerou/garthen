<script setup>
definePageMeta({
  layout: 'app',
})

const { t } = useI18n()
const system = useSystemStore()
const user = useUserStore()

const isGreenhouseAnimationDisabled = ref(true)
const isGreenhouseCreationModalOpened = ref(false)

watchEffect(() => {
  useHead({
    title: t('title'),
  })
})

system.setIsNavbarFolded(true)

onMounted(() => {
  setTimeout(() => (isGreenhouseAnimationDisabled.value = false), 300)
})

onBeforeUnmount(() => system.setIsNavbarFolded(false))
</script>

<template>
  <div class="content">
    <GreenhousesConnectModal
      v-if="isGreenhouseCreationModalOpened"
      @close="isGreenhouseCreationModalOpened = false"
    />
    <div class="heading-container">
      <Transition
        enter-active-class="transition"
        enter-from-class="hide"
        leave-active-class="transition"
        leave-to-class="hide"
        mode="out-in"
      >
        <h3 v-if="user.greenhousesCount === 0">
          {{ t('noGreenhousesHeading') }}
        </h3>
        <h3 v-else>{{ t('heading') }}</h3>
      </Transition>
    </div>

    <Transition enter-from-class="hide" leave-to-class="hide">
      <GreenhousesList :animations-disabled="isGreenhouseAnimationDisabled" />
    </Transition>

    <div class="button-container">
      <Transition
        enter-active-class="transition"
        enter-from-class="hide"
        leave-active-class="transition"
        leave-to-class="hide"
        mode="out-in"
      >
        <GarthenButton
          v-if="user.greenhousesCount === 0"
          @click="isGreenhouseCreationModalOpened = true"
        >
          {{ t('noGreenhousesConnectButton') }}
        </GarthenButton>
        <GarthenButton v-else @click="isGreenhouseCreationModalOpened = true">
          {{ t('connectButton') }}
        </GarthenButton>
      </Transition>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.content {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 1rem;
  min-height: calc(100vh - 1rem * 2);
  padding: 1rem;

  h3 {
    text-align: center;
  }

  .heading-container .hide {
    opacity: 0;
  }

  .button-container .hide {
    opacity: 0;
  }

  .transition {
    transition: opacity var(--fast-transition-duration);
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "title": "My greenhouses",
    "heading": "Select greenhouse",
    "noGreenhousesHeading": "Start by connecting a new greenhouse",
    "connectButton": "Connect new",
    "noGreenhousesConnectButton": "Connect"
  },
  "ru-RU": {
    "title": "Мои теплицы",
    "heading": "Выберите теплицу",
    "noGreenhousesHeading": "Начните с подключения новой теплицы",
    "connectButton": "Подключить новую",
    "noGreenhousesConnectButton": "Подключить"
  }
}
</i18n>
