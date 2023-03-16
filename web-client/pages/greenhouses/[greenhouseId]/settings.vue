<script setup>
definePageMeta({
  layout: 'app',
})

const { $wsSubscribe } = useNuxtApp()
const { t } = useI18n()
const route = useRoute()
const dataStore = useDataStore()
const system = useSystemStore()
const user = useUserStore()

const isSaving = ref(false)
const isDeletionModalOpened = ref(false)

const save = () => {
  isSaving.value = true

  setTimeout(() => (isSaving.value = false), 1500)
}

watchEffect(() => {
  try {
    useHead({
      title: t('fullTitle', {
        greenhouseName:
          dataStore.greenhouses[BigInt(route.params.greenhouseId)].name,
      }),
    })
  } catch (e) {
    useHead({ title: t('title') })
  }
})

onMounted(() => {
  system.setAppPageName('settings')
})

onBeforeUnmount(() => system.setAppPageName(''))
</script>

<template>
  <div class="page">
    <SettingsDeleteGreenhouseModal
      v-if="isDeletionModalOpened"
      @close="isDeletionModalOpened = false"
    />

    <SettingsHeader :saving="isSaving" @save="save" />

    <div class="sections">
      <div class="side">
        <SettingsDangerZone
          :disabled="isSaving"
          @delete="isDeletionModalOpened = true"
        />
      </div>
      <div class="side"></div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.page {
  display: flex;
  flex-direction: column;
  gap: 1.75rem;
  margin: 1.5rem 0 4rem 0;

  @include medium-screen {
    gap: 4rem;
    margin: 2.125rem 0;
  }

  .sections {
    display: flex;
    gap: 2rem;

    @include medium-screen {
      gap: 1.5rem;
    }

    @include large-screen {
      flex-direction: column;
    }

    .side {
      display: flex;
      flex-direction: column;
      gap: 2rem;

      @include medium-screen {
        gap: 1.5rem;
      }

      section {
        display: flex;
        flex-direction: column;
        gap: 1rem;
      }
    }
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "title": "Settings",
    "fullTitle": "Settings - {greenhouseName}"
  },
  "ru-RU": {
    "title": "Настройки",
    "fullTitle": "Настройки - {greenhouseName}"
  }
}
</i18n>
