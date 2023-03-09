<script setup>
definePageMeta({
  layout: 'app',
})

const { $wsSubscribe } = useNuxtApp()
const { t } = useI18n()
const route = useRoute()
const dataStore = useDataStore()
const system = useSystemStore()

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
  system.setAppPageName('analytics')
})

onBeforeUnmount(() => system.setAppPageName(''))
</script>

<template>
  <div class="page">
    <AnalyticsHeader />
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
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "title": "Analytics",
    "fullTitle": "Analytics - {greenhouseName}"
  },
  "ru-RU": {
    "title": "Аналитика",
    "fullTitle": "Аналитика - {greenhouseName}"
  }
}
</i18n>
