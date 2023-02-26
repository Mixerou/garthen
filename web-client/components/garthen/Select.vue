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
      @click="isOpened = disabled ? isOpened : !isOpened"
    >
      {{
        selected === null ? '' : items.find(item => item.id === selected).value
      }}
    </span>
    <Transition enter-from-class="hide" leave-to-class="hide">
      <div v-if="isOpened && !disabled" class="options">
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

    &.opened:not(.disabled) {
      transform: translateY(-0.125rem);

      @include medium-screen {
        transform: translateY(-0.25rem);
      }
    }

    &:not(.opened):not(.disabled) {
      &:hover {
        background: var(--primary-500);
      }

      &:active {
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
    left: 0;
    min-width: calc(100% - 0.625rem * 2);
    padding: 0.5rem 0.5rem;
    border: var(--default-border);
    border-color: var(--white-original);
    border-radius: var(--large-radius);
    transform: translateY(calc(100% + 0.25rem));
    background: var(--primary-400);
    color: var(--primary-layer-0-color);
    transition: var(--fast-transition-duration);

    &.hide {
      opacity: 0;
      transform: translateY(calc(100% + 1rem));
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
