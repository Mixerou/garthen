<script setup>
definePageMeta({
  layout: 'app',
})

const { t } = useI18n()
const route = useRoute()
const dataStore = useDataStore()
const system = useSystemStore()
const user = useUserStore()

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

onMounted(() => system.setAppPageName('dashboard'))
onBeforeUnmount(() => system.setAppPageName(''))
</script>

<template>
  <div class="page">
    <DashboardHeader />
  </div>
</template>

<style lang="scss" scoped>
.page {
  margin-top: 1.5rem;

  @include medium-screen {
    margin-top: 2.125rem;
  }
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "title": "Dashboard",
    "fullTitle": "Dashboard - {greenhouseName}",
    "heading": "Dashboard",
    "headingWithGreeting": "Welcome back, #!"
  },
  "ru-RU": {
    "title": "Панель управления",
    "fullTitle": "Панель управления - {greenhouseName}",
    "heading": "Панель управления",
    "headingWithGreeting": "С возвращением, #!"
  }
}
</i18n>
