<script setup>
const { t } = useI18n()
</script>

<template>
  <div class="badge">{{ t('later') }}</div>
</template>

<style lang="scss" scoped>
.badge {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 1.5rem;
  padding: 0 0.5rem;
  border-radius: var(--default-radius);
  background: var(--primary);
  font-size: var(--small-font-size);
  font-weight: 600;
  text-transform: uppercase;
  color: var(--white-original);
  transition: background-color var(--default-transition);
}
</style>

<i18n lang="json">
{
  "en-GB": {
    "later": "Later"
  },
  "ru-RU": {
    "later": "Позже"
  }
}
</i18n>
