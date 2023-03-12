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
  layout: {
    type: String,
    required: true,
    validator(layout) {
      return ['charts', 'tables'].includes(layout)
    },
  },
})
const emit = defineEmits(['select:range', 'update:layout'])
</script>

<template>
  <header>
    <AnalyticsHeaderLeftSide
      :loading="loading"
      :range="range"
      @select:range="range => emit('select:range', range)"
    />
    <AnalyticsHeaderRightSide
      :layout="layout"
      @update:layout="layout => emit('update:layout', layout)"
    />
  </header>
</template>

<style lang="scss" scoped>
header {
  display: flex;
  justify-content: space-between;
  gap: 0.25rem;
}
</style>
