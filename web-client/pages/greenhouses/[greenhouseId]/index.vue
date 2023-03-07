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

const isDisabledDevicesModalOpened = ref(false)

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

watchEffect(() => {
  $wsSubscribe(
    'devices',
    {
      a: 'subscribe_to_devices_update',
      greenhouse_id: BigInt(route.params.greenhouseId || 0),
    },
    true
  )
})

onMounted(() => {
  system.setAppPageName('dashboard')
})

onBeforeUnmount(() => system.setAppPageName(''))
</script>

<template>
  <div class="page">
    <DashboardDisabledCardsModal
      v-if="isDisabledDevicesModalOpened"
      @close="isDisabledDevicesModalOpened = false"
    />

    <DashboardHeader />
    <DashboardGroups />

    <GarthenButton @click="isDisabledDevicesModalOpened = true">{{
      t('buttons.showDisabled')
    }}</GarthenButton>
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

  button {
    width: fit-content;
    margin: 0 auto;
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "title": "Dashboard",
    "fullTitle": "Dashboard - {greenhouseName}",
    "buttons": {
      "showDisabled": "Show disabled"
    }
  },
  "ru-RU": {
    "title": "Панель управления",
    "fullTitle": "Панель управления - {greenhouseName}",
    "buttons": {
      "showDisabled": "Показать отключенные"
    }
  }
}
</i18n>
