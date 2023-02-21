<script setup>
import IconPsychiatry from '@/assets/icons/psychiatry.svg?skipsvgo'

definePageMeta({
  layout: 'app',
})

const { t } = useI18n()
const dataStore = useDataStore()
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
</script>

<template>
  <div class="content">
    <GarthenModal
      v-if="isGreenhouseCreationModalOpened"
      close-on-click-outside
      @close="isGreenhouseCreationModalOpened = false"
    >
      <GreenhousesCreateModal />
    </GarthenModal>
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
      <div v-if="user.greenhousesCount !== 0" class="greenhouses">
        <TransitionGroup
          enter-active-class="transition"
          enter-from-class="hide"
          leave-active-class="transition"
          leave-to-class="hide"
        >
          <NuxtLink
            v-for="greenhouse in dataStore.greenhouses"
            :key="`greenhouse-${greenhouse.id}`"
            class="greenhouse"
            :class="{
              ['disable-transition-animation']: isGreenhouseAnimationDisabled,
            }"
            :to="`/greenhouses/${greenhouse.id}`"
          >
            <GarthenButton>
              <IconPsychiatry class="icon" />
              <span>{{ greenhouse.name }}</span>
            </GarthenButton>
          </NuxtLink>
        </TransitionGroup>
      </div>
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
          {{ $t('noGreenhousesConnectButton') }}
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

  .greenhouses {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    width: 16rem;
    max-height: 20rem;
    padding: 0.25rem 0.5rem;
    border-radius: var(--large-radius);
    background: var(--primary);
    overflow-y: auto;
    transition: var(--default-transition);

    @include medium-screen {
      gap: 0.5rem;
      width: 20rem;
      padding: 0.5rem 1rem;
    }

    &.hide {
      max-height: 0;
      margin-bottom: calc(-1rem - 0.25rem * 2);
      opacity: 0;

      @include medium-screen {
        margin-bottom: calc(-1rem - 0.5rem * 2);
      }
    }

    .greenhouse {
      border-radius: var(--large-radius);
      text-decoration: none;
      width: 100%;

      &.hide:not(.disable-transition-animation) {
        margin-top: calc(-2.5rem - 0.25rem);
        opacity: 0;

        @include medium-screen {
          margin-top: calc(-2.75rem - 0.5rem);
        }
      }

      &.transition {
        transition: var(--default-transition);
      }

      button {
        width: calc(100% - 2rem - 1rem * 2);

        @include medium-screen {
          width: calc(100% - 1.5rem - 1rem * 2);
        }
      }

      .icon {
        width: 1.5rem;

        @include medium-screen {
          width: 1.75rem;
        }
      }
    }
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
