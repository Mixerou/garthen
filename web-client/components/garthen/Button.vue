<script setup>
defineProps({
  disabled: {
    type: Boolean,
    required: false,
    default: false,
  },
})
</script>

<template>
  <button :disabled="disabled">
    <div class="content">
      <slot />
    </div>
  </button>
</template>

<style lang="scss" scoped>
button {
  all: unset;
  position: relative;
  height: 2.5rem;
  padding: 0 2rem;
  border-radius: var(--large-radius);
  overflow: hidden;
  background: var(--primary-400);
  cursor: pointer;
  font-size: var(--default-font-size);
  font-weight: 600;
  color: var(--white-original);
  fill: var(--white-original);
  transition-duration: var(--fast-transition-duration);
  transition-property: background-color, color, fill;

  @include medium-screen {
    height: 2.75rem;
    padding: 0 1.75rem;
  }

  &:not(:disabled) {
    &:hover,
    &:focus-visible {
      background: var(--primary-500);
    }

    &:active {
      background: var(--primary-600);
      color: var(--white-original-100);
      fill: var(--white-original-100);
    }
  }

  &:disabled {
    background: var(--primary-500);
    cursor: not-allowed;
  }

  .content {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: var(--fast-transition-duration);
  }
}
</style>
