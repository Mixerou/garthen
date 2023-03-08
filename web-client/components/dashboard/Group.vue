<script setup>
defineProps({
  name: {
    type: String,
    required: true,
  },
})
</script>

<template>
  <div class="group">
    <div class="meta">
      <h6>{{ name }}</h6>
    </div>
    <div class="cards">
      <slot />
    </div>
  </div>
</template>

<style lang="scss" scoped>
.group {
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 1.5rem;
  width: fit-content;
  max-width: 100%;
  padding: 1rem;
  border-radius: var(--large-radius);
  overflow: hidden;
  background: var(--white);
  transition: background-color var(--default-transition);

  @include medium-screen {
    gap: 2rem;
    max-width: calc(100% - 2rem);
    padding: 2rem;
  }

  .cards {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    justify-content: center;
    gap: 1.5rem;

    @include medium-screen {
      gap: 2rem;
      flex-direction: row;
    }
  }
}
</style>
