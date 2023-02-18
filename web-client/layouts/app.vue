<script setup>
const router = useRouter()
const system = useSystemStore()
const user = useUserStore()

const isLoading = ref(true)
const isNavbarVisible = ref(false)

system.setIsAppLayout(true)

onMounted(() => {
  watchEffect(() => {
    if (!user.isLoggedIn) router.push('/')
  })

  isLoading.value = false
  setTimeout(() => (isNavbarVisible.value = true), 1)
})

onBeforeUnmount(() => {
  system.setIsAppLayout(false)
})
</script>

<template>
  <div id="app-layout">
    <Transition>
      <div v-if="isLoading" class="loader-container">
        <GarthenLoader class="loader" :stop="!isLoading" />
      </div>
      <div v-else class="content">
        <Navbar
          :class="{ hide: !isNavbarVisible }"
          :fold="system.isNavbarFolded"
        />
        <main>
          <slot />
        </main>
      </div>
    </Transition>
  </div>
</template>

<style lang="scss" scoped>
#app-layout {
  width: 100%;

  .loader-container {
    position: fixed;
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 500;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: var(--layer-0-background);

    .loader {
      width: 2rem;
    }
  }

  .v-enter-from {
    opacity: 0;
    transform: scale(0.75);
  }

  .v-leave-to {
    opacity: 0;
    transform: scale(1.25);
  }

  .v-enter-active,
  .v-leave-active {
    transition: var(--default-transition);
  }

  .content {
    display: flex;
    width: 100%;

    nav {
      transition: transform var(--default-transition);

      &.hide {
        transform: translateX(-200%) scale(1.25);
      }
    }

    main {
      width: 100%;
    }
  }
}
</style>
