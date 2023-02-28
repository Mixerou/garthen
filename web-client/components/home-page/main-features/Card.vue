<script setup>
defineProps({
  withLaterBadge: {
    type: Boolean,
    required: false,
    default: false,
  },
})
</script>

<template>
  <div class="card">
    <div class="icon">
      <slot name="icon" />
    </div>
    <div class="heading">
      <h5>
        <slot name="heading" />
      </h5>
      <HomePageBadge v-if="withLaterBadge" />
    </div>
    <p>
      <slot name="description" />
    </p>
  </div>
</template>

<style lang="scss" scoped>
.card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  width: 26rem;
  max-width: calc(100vw - 1rem * 2);
  text-align: center;

  .icon {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 3.5rem;
    height: 3.5rem;
    border-radius: 100%;
    background: #e9f0ee;

    @include medium-screen {
      width: 3.75rem;
      height: 3.75rem;
    }

    :deep(svg) {
      width: 1.5rem;
      fill: var(--primary);

      @include medium-screen {
        width: 1.75rem;
      }
    }
  }

  .heading {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  p {
    line-height: var(--default-line-height);
    font-size: var(--default-font-size);
  }
}
</style>
