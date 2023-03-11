<script setup>
defineProps({
  loading: {
    type: Boolean,
    required: false,
    default: false,
  },
  range: {
    type: Number,
    required: true,
  },
})
const emit = defineEmits(['select:range'])
</script>

<template>
  <header>
    <AnalyticsHeaderLeftSide
      :loading="loading"
      :range="range"
      @select:range="range => emit('select:range', range)"
    />
    <AnalyticsHeaderRightSide />
  </header>
</template>

<style lang="scss" scoped>
header {
  display: flex;
  justify-content: space-between;
  gap: 0.25rem;
}
</style>
