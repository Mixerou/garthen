<script setup>
const props = defineProps({
  closeOnClickOutside: {
    type: Boolean,
    required: false,
    default: false,
  },
  isDropdownVisible: {
    type: Boolean,
    required: false,
    default: false,
  },
  disableDropdownPadding: {
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

onBeforeUnmount(() => {
  if (system.modals.indexOf(id) >= 0) {
    system.unregisterActiveModal()
  }
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
        <Transition enter-from-class="hide" leave-to-class="hide">
          <div
            v-if="isDropdownVisible"
            class="dropdown"
            :class="{ ['disable-padding']: disableDropdownPadding }"
          >
            <slot name="dropdown" />
          </div>
        </Transition>
        <div class="content">
          <slot name="content" />
        </div>
      </div>

      <slot name="custom" />
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

  .dropdown {
    position: absolute;
    display: flex;
    flex-direction: column;
    align-items: center;
    bottom: 1rem;
    left: 0;
    width: 18rem;
    max-width: calc(100vw - 5rem);
    padding: 1rem 1.5rem;
    border-radius: var(--large-radius);
    box-shadow: var(--large-shadow);
    background: var(--primary);
    color: var(--primary-layer-0-color);
    transform: translateY(calc(100% + 1.5rem));
    transition: transform var(--default-transition);

    &.disable-padding {
      left: 0.5rem;
      width: 20rem;
      max-width: calc(100vw - 3rem);
      padding: 0;

      @include medium-screen {
        left: 0.25rem;
        width: 20.5rem;
      }
    }

    &.hide {
      box-shadow: unset;
      transform: translateY(0);
    }
  }

  .content {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    z-index: 125;
    width: 18rem;
    max-width: calc(100vw - 5rem);
    padding: 1rem 1.5rem;
    border-radius: var(--large-radius);
    box-shadow: var(--large-shadow);
    background: var(--primary);
    font-size: var(--default-font-size);
    color: var(--primary-layer-0-color);

    &:deep(h5) {
      max-width: 100%;
      overflow: hidden;
      white-space: nowrap;
      font-weight: 900;
      text-overflow: ellipsis;
    }

    &:deep(input) {
      border-color: #ffffff66;
      color: var(--primary-layer-0-color);

      &:focus {
        border-color: #ffffff;
      }

      &::placeholder {
        color: var(--primary-layer-0-color);
      }

      &.v-enter-active,
      &.v-leave-active {
        transition: var(--default-transition);
      }
    }

    &:deep(.select) {
      .selected-value {
        border-color: #ffffff66;
        color: var(--primary-layer-0-color);

        &.opened:not(.disabled) {
          border-color: #ffffff;
        }
      }

      .options {
        border-color: #ffffff;
        color: var(--primary-layer-0-color);
      }
    }
  }
}

.hide {
  transform: translateX(calc(100% + 4rem));
}
</style>
