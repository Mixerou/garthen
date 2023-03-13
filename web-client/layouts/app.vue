<script setup>
const { $wsSend, $wsSubscribe } = useNuxtApp()
const router = useRouter()
const constants = useConstantsStore()
const system = useSystemStore()
const user = useUserStore()

const isLoading = ref(true)
const isNavbarVisible = ref(false)

system.setIsAppLayout(true)

onMounted(() => {
  watchEffect(() => {
    if (!user.isLoggedIn) router.push('/')
  })

  watchEffect(() => {
    const state = user.id !== 0 && system.isWebSocketConnected

    isLoading.value = !state

    setTimeout(() => (isNavbarVisible.value = state), 1)

    // TODO: Check is already subscribed
    $wsSubscribe('greenhouses/mine', {}, true)
    $wsSubscribe('greenhouse-create', {}, true)
  })
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
    z-index: 1500;
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
    transform: scale(0.95);
  }

  .v-leave-to {
    opacity: 0;
    transform: scale(1.05);
  }

  .v-enter-active,
  .v-leave-active {
    transition: var(--default-transition);
  }

  .content {
    display: flex;
    width: 100%;
    height: 100vh;

    nav.hide {
      transform: translateX(-200%) scale(1.05);
    }

    main {
      width: 100%;
      height: 100%;
      padding: 0 1rem;
      overflow-x: hidden;
      overflow-y: auto;

      @include medium-screen {
        padding: 0 2rem;
      }
    }
  }
}
</style>
