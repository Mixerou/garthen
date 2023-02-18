<script setup>
defineProps({
  fold: {
    type: Boolean,
    required: false,
    default: false,
  },
})
</script>

<template>
  <nav :class="{ fold }">
    <GarthenLogo :small="fold" />
    <div class="buttons"></div>
    <Profile class="profile" :avatar-only="fold" />
  </nav>
</template>

<style lang="scss" scoped>
nav {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4rem;
  width: 20rem;
  height: calc(100vh - 1.5rem);
  padding-top: 1.5rem;
  background: #e9f0ee;

  @include medium-screen {
    height: calc(100vh - 2rem);
    padding: 2rem 2rem 0;
  }

  &.fold {
    width: 5.5rem;

    @include medium-screen {
      width: calc(7.75rem - 2rem * 2);
    }
  }

  .buttons {
    height: 100%;
  }

  .profile {
    margin-bottom: 4rem;

    @include medium-screen {
      margin-bottom: 1rem;
    }
  }
}

body[data-theme='dark'] nav {
  background: #1c2b31;
}
</style>
