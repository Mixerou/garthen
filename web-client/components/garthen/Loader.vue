<script setup>
import IconFlower from '@/assets/icons/flower.svg?skipsvgo'

defineProps({
  stop: {
    type: Boolean,
    required: false,
    default: false,
  },
})
</script>

<template>
  <div class="loader" :class="{ stop }">
    <IconFlower />
  </div>
</template>

<style lang="scss" scoped>
.loader:not(.stop) svg {
  animation: spin 1s infinite;
}

@keyframes spin {
  from {
    transform: rotate(0);
  }

  to {
    transform: rotate(360deg);
  }
}
</style>
