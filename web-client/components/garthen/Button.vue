<script setup>
defineProps({
  disabled: {
    type: Boolean,
    required: false,
    default: false,
  },
  loading: {
    type: Boolean,
    required: false,
    default: false,
  },
  transparentBackground: {
    type: Boolean,
    required: false,
    default: false,
  },
})
</script>

<template>
  <button
    :class="{ loading, ['transparent-background']: transparentBackground }"
    :disabled="disabled || loading"
  >
    <GarthenLoader class="loader" :stop="!loading" />
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
  transition: background-color var(--fast-transition-duration);

  @include medium-screen {
    height: 2.75rem;
    padding: 0 1.75rem;
  }

  &.transparent-background {
    background: transparent;

    &:deep(*:not(.loader *)) {
      color: var(--white-900);
      fill: var(--white-900);
    }
  }

  &:not(.loading):not(:disabled) {
    &:hover,
    &:focus-visible {
      background: var(--primary-500);

      &:deep(*:not(.loader *)) {
        color: var(--white-original);
        fill: var(--white-original);
      }
    }

    &:active {
      background: var(--primary-600);

      &:deep(*:not(.loader *)) {
        color: var(--white-original-100);
        fill: var(--white-original-100);
      }
    }
  }

  &:disabled {
    background: var(--primary-500);

    &:not(.loading) {
      cursor: not-allowed;
    }
  }

  &.loading {
    cursor: wait;

    .content {
      opacity: 0;
      transform: scale(0.95);
    }

    .loader {
      transform: translate(-50%, -50%);
      opacity: 1;
    }
  }

  &:deep(*:not(.loader *)) {
    color: var(--white-original);
    fill: var(--white-original);
    transition: background-color var(--fast-transition-duration),
      color var(--fast-transition-duration), fill 0.1s;
  }

  .content {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: var(--fast-transition-duration);
  }

  .loader {
    position: absolute;
    top: 50%;
    left: 50%;
    width: 1.25rem;
    height: 1.25rem;
    opacity: 0;
    transform: translate(-50%, -50%) scale(1.25);
    transition: var(--fast-transition-duration);

    @include medium-screen {
      width: 1.5rem;
      height: 1.5rem;
    }
  }
}
</style>
