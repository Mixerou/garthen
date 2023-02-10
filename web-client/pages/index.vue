<script setup>
useSeoMeta({
  title: '',
})

const { t } = useI18n()
</script>

<template>
  <h1>{{ t('pageName') }}</h1>
</template>

<style scoped></style>

<i18n lang="json">
{
  "en-GB": {
    "pageName": "Home page"
  },
  "ru-RU": {
    "pageName": "Домашняя страница"
  }
}
</i18n>
