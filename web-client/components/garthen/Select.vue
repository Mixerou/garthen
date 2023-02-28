<script setup>
const props = defineProps({
  items: {
    type: Array,
    required: false,
    default: () => [],
    validator(values) {
      for (const value of values) {
        if (value.id === undefined) return false
      }

      return true
    },
  },
  selected: {
    type: String,
    required: false,
    default: null,
  },
  disabled: {
    type: Boolean,
    required: false,
    default: false,
  },
  dropdownDirection: {
    type: String,
    required: false,
    default: 'bottom',
    validator(direction) {
      return ['top', 'bottom'].includes(direction)
    },
  },
})
const emit = defineEmits(['update:selected'])

const isOpened = ref(false)

const computedSelected = computed({
  get() {
    return props.selected.value
  },
  set(value) {
    if (props.disabled.value) return

    emit('update:selected', value)
  },
})

const onSelect = id => {
  if (props.disabled) return

  computedSelected.value = id
  isOpened.value = false
}
</script>

<template>
  <div v-click-outside="() => (isOpened = false)" class="select">
    <span
      class="selected-value"
      :class="{ opened: isOpened, disabled }"
      :data-dropdown-direction="props.dropdownDirection"
      @click="isOpened = disabled ? isOpened : !isOpened"
    >
      {{
        selected === null ? '' : items.find(item => item.id === selected).value
      }}
    </span>
    <Transition enter-from-class="hide" leave-to-class="hide">
      <div
        v-if="isOpened && !disabled"
        class="options"
        :data-direction="props.dropdownDirection"
      >
        <div
          v-for="item in items"
          :key="`option-${item.id}`"
          class="option"
          :class="{ ['selected-option']: item.id === selected }"
          role="option"
          @click="onSelect(item.id)"
        >
          {{ item.value }}
        </div>
      </div>
    </Transition>
  </div>
</template>

<style lang="scss" scoped>
.select {
  position: relative;
  font-size: var(--default-font-size);
  font-weight: 600;
  color: var(--layer-0-color);
  transition-duration: var(--fast-transition-duration);

  .selected-value {
    position: relative;
    display: flex;
    align-items: center;
    height: 2rem;
    padding: 0 1rem;
    border: var(--default-border);
    border-radius: var(--large-radius);
    cursor: pointer;
    transition-duration: var(--fast-transition-duration);

    @include medium-screen {
      height: 2.25rem;
    }

    &.disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }

    &[data-dropdown-direction='top'].opened:not(.disabled) {
      transform: translateY(0.125rem);

      @include medium-screen {
        transform: translateY(0.25rem);
      }
    }

    &[data-dropdown-direction='bottom'].opened:not(.disabled) {
      transform: translateY(-0.125rem);

      @include medium-screen {
        transform: translateY(-0.25rem);
      }
    }

    &:not(.opened):not(.disabled) {
      &:hover {
        background: var(--primary-500);
      }

      &[data-dropdown-direction='top']:active {
        transform: translateY(-0.125rem);
        background: var(--primary-600);

        @include medium-screen {
          transform: translateY(-0.25rem);
        }
      }

      &[data-dropdown-direction='bottom']:active {
        transform: translateY(0.125rem);
        background: var(--primary-600);

        @include medium-screen {
          transform: translateY(0.25rem);
        }
      }
    }
  }

  .options {
    position: absolute;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    z-index: 999;
    bottom: 0;
    left: 50%;
    min-width: calc(100% - 0.625rem * 2);
    padding: 0.5rem 0.5rem;
    border: var(--default-border);
    border-color: var(--white-original);
    border-radius: var(--large-radius);
    background: var(--primary-400);
    color: var(--primary-layer-0-color);
    transition: var(--fast-transition-duration);

    &[data-direction='top'] {
      transform: translate(-50%, calc(-2rem - 0.5rem));

      @include medium-screen {
        transform: translate(-50%, calc(-2.25rem - 0.5rem));
      }

      &.hide {
        opacity: 0;
        transform: translate(-50%, calc(-2rem - 1.25rem));

        @include medium-screen {
          transform: translate(-50%, calc(-2.25rem - 1.25rem));
        }
      }
    }

    &[data-direction='bottom'] {
      transform: translate(-50%, calc(100% + 0.25rem));

      &.hide {
        opacity: 0;
        transform: translate(-50%, calc(100% + 1rem));
      }
    }

    .option {
      display: flex;
      align-items: center;
      width: calc(100% - 1rem * 2);
      padding: 0.25rem 1rem;
      border-radius: var(--medium-radius);
      background: var(--primary-400);
      cursor: pointer;
      transition: var(--fast-transition-duration);

      &:hover,
      &.selected-option {
        background: var(--primary-500);
      }

      &:active {
        background: var(--primary-600);
      }
    }
  }
}

.select {
}
</style>
