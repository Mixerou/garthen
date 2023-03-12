<script setup>
import IconCheck from '@/assets/icons/check.svg?skipsvgo'

const props = defineProps({
  checked: {
    type: Boolean,
    required: false,
    default: false,
  },
})
// TODO: Maybe make `update:state`
const emit = defineEmits(['update:checked'])
</script>

<template>
  <div
    class="checkbox"
    :class="{ checked }"
    role="checkbox"
    @click="emit('update:checked', !checked)"
  >
    <Transition enter-from-class="hide" leave-to-class="hide">
      <IconCheck v-if="checked" />
    </Transition>
  </div>
</template>

<style lang="scss" scoped>
.checkbox {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 1.25rem;
  height: 1.25rem;
  border: var(--default-border);
  border-color: var(--primary-400);
  border-radius: var(--default-radius);
  cursor: pointer;
  transition: var(--fast-transition-duration);

  &:hover {
    border-color: var(--primary-500);
  }

  &:active {
    border-color: var(--primary-600);
  }

  @include medium-screen {
    width: 1.5rem;
    height: 1.5rem;
    border-radius: var(--medium-radius);
  }

  &.checked {
    border-color: transparent;
    background: var(--primary);

    &:hover {
      background: var(--primary-500);
    }

    &:active {
      background: var(--primary-600);

      svg {
        fill: var(--white-original-100);
      }
    }
  }

  svg {
    width: 1rem;
    fill: var(--white-original);
    transition: var(--fast-transition-duration);

    @include medium-screen {
      width: 1.25rem;
    }

    &.hide {
      opacity: 0;
    }
  }
}
</style>
