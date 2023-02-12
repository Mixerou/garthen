<script setup>
const props = defineProps({
  closeOnClickOutside: {
    type: Boolean,
    required: false,
    default: false,
  },
})
const emit = defineEmits(['close'])

const system = useSystemStore()

const id = parseInt(`${Date.now()}${Math.floor(Math.random() * 100)}`)
const isMounted = ref(false)

const onClickOutside = () => {
  if (
    isMounted.value &&
    props.closeOnClickOutside &&
    system.modals.indexOf(id) === 0
  ) {
    system.unregisterActiveModal()
  }
}

watchEffect(() => {
  if (isMounted.value && system.modals.indexOf(id) < 0) {
    const timeoutTime =
      getComputedStyle(document.body)
        .getPropertyValue('--default-transition-duration')
        .split('s')[0] * 1000

    setTimeout(() => emit('close'), timeoutTime)
  }
})

onMounted(() => {
  system.registerModal(id)
  setTimeout(() => (isMounted.value = true), 10)
})
</script>

<template>
  <ClientOnly>
    <Teleport to="#modals">
      <div
        v-click-outside="onClickOutside"
        :class="{ hide: system.modals.indexOf(id) !== 0 || !isMounted }"
        class="modal"
      >
        <slot />
      </div>
    </Teleport>
  </ClientOnly>
</template>

<style lang="scss" scoped>
.modal {
  position: fixed;
  z-index: 100;
  top: 1.5rem;
  right: 1rem;
  transition: var(--default-transition);
  transform: translateX(0);
}

.hide {
  transform: translateX(calc(100% + 4rem));
}
</style>
