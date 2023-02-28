<script setup>
defineProps({
  withLaterBadge: {
    type: Boolean,
    required: false,
    default: false,
  },
  reverse: {
    type: Boolean,
    required: false,
    default: false,
  },
})
</script>

<template>
  <section :class="{ reverse }">
    <div class="information">
      <div class="heading">
        <h2>
          <slot name="heading" />
        </h2>
        <HomePageBadge v-if="withLaterBadge" />
      </div>
      <p>
        <slot name="description" />
      </p>
    </div>

    <div class="image">
      <slot name="image" />
      <div class="background" />
    </div>
  </section>
</template>

<style lang="scss" scoped>
section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2rem;
  padding: 2rem 0;

  @include medium-screen {
    padding: 4rem 0;
  }

  @include xxl-screen {
    flex-direction: row;
    padding: 6rem 0;

    &.reverse {
      flex-direction: row-reverse;
    }
  }

  .information {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 1rem;
    width: 100%;
    text-align: center;

    .heading {
      display: flex;
      align-items: center;
      gap: 1rem;
    }

    p {
      max-width: 44rem;
      line-height: var(--medium-line-height);
      font-size: var(--medium-font-size);
    }
  }

  .image {
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;

    :deep(svg) {
      height: 10rem;
      z-index: 1;

      @include medium-screen {
        height: 16.75rem;
      }
    }

    .background {
      position: absolute;
      bottom: 0;
      width: 44rem;
      max-width: calc(100vw - 1rem * 2);
      height: 5rem;
      border-radius: var(--large-radius);
      background: var(--primary);

      @include medium-screen {
        height: 8.5rem;
      }
    }
  }
}
</style>
