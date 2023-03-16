<script setup>
defineProps({
  saving: {
    type: Boolean,
    required: false,
    default: false,
  },
})
const emit = defineEmits(['save'])
</script>

<template>
  <header>
    <SettingsHeaderLeftSide :saving="saving" @save="emit('save')" />
    <SettingsHeaderRightSide />
  </header>
</template>

<style lang="scss" scoped>
header {
  display: flex;
  justify-content: space-between;
  gap: 0.25rem;
}
</style>
