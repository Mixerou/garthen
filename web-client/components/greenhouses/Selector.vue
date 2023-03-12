<script setup>
import IconExpandMore from '@/assets/icons/expand-more.svg?skipsvgo'
import IconAdd from '@/assets/icons/add.svg?skipsvgo'

const dataStore = useDataStore()

const isCloseBlocked = ref(false)
const animateSelector = ref(false)
const isGreenhouseCreationModalOpened = ref(false)

const selectorPosition = ref('absolute')
const selectorZIndex = ref('initial')
const selectorTransition = ref('none')

const invisibleSelector = ref(null)
const invisibleSelectorTop = ref('0')
const invisibleSelectorLeft = ref('0')
const invisibleSelectorWidth = ref('auto')

const greenhousesList = ref(null)
const isGreenhousesListOpened = ref(false)
const greenhouseListHeight = ref('0')

let animationTimeout = null

const open = () => {
  clearTimeout(animationTimeout)

  const invisibleSelectorRect = invisibleSelector.value.getBoundingClientRect()

  isCloseBlocked.value = true
  isGreenhousesListOpened.value = true
  selectorPosition.value = 'fixed'
  selectorZIndex.value = '999'
  invisibleSelectorTop.value = `${invisibleSelectorRect.top}px`
  invisibleSelectorLeft.value = `${invisibleSelectorRect.left}px`
  invisibleSelectorWidth.value = `${invisibleSelectorRect.width}px`

  animationTimeout = setTimeout(() => {
    const greenhousesListRect = greenhousesList.value.getBoundingClientRect()

    animateSelector.value = true
    selectorTransition.value = 'var(--default-transition)'
    invisibleSelectorLeft.value = `${greenhousesListRect.left}px`
    greenhouseListHeight.value = `${greenhousesListRect.height}px`
  }, 1)
}

const close = () => {
  clearTimeout(animationTimeout)

  if (!isGreenhousesListOpened.value) return

  const invisibleSelectorRect = invisibleSelector.value.getBoundingClientRect()

  isGreenhousesListOpened.value = false
  invisibleSelectorTop.value = `${invisibleSelectorRect.top}px`
  invisibleSelectorLeft.value = `${invisibleSelectorRect.left}px`
  invisibleSelectorWidth.value = `${invisibleSelectorRect.width}px`

  animationTimeout = setTimeout(() => {
    animateSelector.value = false

    animationTimeout = setTimeout(() => {
      selectorPosition.value = 'absolute'
      selectorZIndex.value = 'initial'
      selectorTransition.value = 'none'
      invisibleSelectorTop.value = `0`
      invisibleSelectorLeft.value = `0`
      invisibleSelectorWidth.value = `auto`
      isCloseBlocked.value = false
    }, 700)
  }, 1)
}

const openModal = () => {
  close()

  isGreenhouseCreationModalOpened.value = true
}

const onResize = () => {
  if (!isGreenhousesListOpened.value) return

  open()
}

onMounted(() => {
  window.addEventListener('resize', onResize)
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', onResize)
})
</script>

<template>
  <div v-click-outside="close" class="container">
    <GreenhousesConnectModal
      v-if="isGreenhouseCreationModalOpened"
      @close="isGreenhouseCreationModalOpened = false"
    />

    <Transition enter-from-class="hide" leave-to-class="hide">
      <div
        v-if="isGreenhousesListOpened"
        ref="greenhousesList"
        class="greenhouses-list"
      >
        <GreenhousesList
          :excluded-greenhouses="[
            dataStore.greenhouses[BigInt($route.params.greenhouseId)]?.id,
          ]"
        />
      </div>
    </Transition>

    <div class="selector" :class="{ opened: animateSelector }">
      <div class="add-button">
        <GarthenButton @click="openModal">
          <IconAdd class="add-icon" />
        </GarthenButton>
      </div>

      <GarthenButton
        class="open-button"
        @click="isGreenhousesListOpened ? close() : open()"
      >
        <IconExpandMore class="icon" />
        <span>
          {{ dataStore.greenhouses[BigInt($route.params.greenhouseId)]?.name }}
        </span>
      </GarthenButton>
    </div>

    <div ref="invisibleSelector" class="selector invisible">
      <GarthenButton class="icon">
        <IconExpandMore class="icon" />
        <span>
          {{ dataStore.greenhouses[BigInt($route.params.greenhouseId)]?.name }}
        </span>
      </GarthenButton>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.container {
  position: relative;

  .greenhouses-list {
    z-index: 999;
    position: fixed;
    top: 1.5rem;
    right: 1em;
    transition: var(--default-transition);

    @include medium-screen {
      top: 2.125rem;
      right: 2em;
    }

    &.hide {
      opacity: 1;
      transform: translateY(calc(-100% - 4rem));
    }

    .greenhouses {
      box-shadow: var(--large-shadow);
    }
  }

  .selector {
    display: flex;
    justify-content: flex-end;
    border-radius: var(--large-radius);
    transition: v-bind(selectorTransition);

    &.invisible {
      opacity: 0;
      pointer-events: none;
    }

    &:not(.invisible) {
      position: v-bind(selectorPosition);
      z-index: v-bind(selectorZIndex);
      top: v-bind(invisibleSelectorTop);
      left: v-bind(invisibleSelectorLeft);
      width: v-bind(invisibleSelectorWidth);

      &.opened {
        top: calc(1.5rem + 0.5rem + v-bind(greenhouseListHeight));
        left: calc(v-bind(invisibleSelectorLeft) + 2.75rem + 0.5rem);
        width: calc(16rem - 1.25rem - 0.5rem * 2);
        box-shadow: var(--large-shadow);

        @include medium-screen {
          top: calc(2.125rem + 0.5rem + v-bind(greenhouseListHeight));
          width: calc(20rem - 0.25rem - 0.5rem * 2);
        }

        .add-button {
          left: calc(-2.75rem - 0.5rem);
          border-radius: var(--large-radius);
          box-shadow: var(--large-shadow);
        }

        .icon {
          transform: rotate(180deg);
        }

        span {
          margin-right: 0;
          opacity: 1;
        }
      }
    }

    .add-button {
      position: absolute;
      top: 0;
      left: 0;
      transition: var(--default-transition);

      button {
        display: flex;
        justify-content: center;
        align-items: center;
        width: 2.75rem;
        padding: 0;
      }
    }

    .open-button {
      width: calc(100% - 1.25rem * 2);
      padding: 0 1.25rem;

      @include medium-screen {
        width: calc(100% - 1.5rem * 2);
        padding: 0 1.5rem;
      }
    }

    button {
      padding: 0 1.25rem;

      @include medium-screen {
        padding: 0 1.5rem;
      }

      .content {
        display: flex;
        align-items: center;
        gap: 0.5rem;

        svg {
          width: 1.5rem;
          transition: var(--default-transition);

          @include medium-screen {
            width: 1.75rem;
          }
        }

        span {
          width: 0;
          margin-right: -0.5rem;
          opacity: 0;
          white-space: nowrap;
          transition: var(--default-transition);

          @include large-screen {
            width: fit-content;
            margin-right: 0;
            opacity: 1;
          }
        }
      }
    }
  }
}
</style>
