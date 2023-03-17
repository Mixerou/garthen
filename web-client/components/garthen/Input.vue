<script setup>
const props = defineProps({
  placeholder: {
    type: String,
    required: false,
    default: '',
  },
  type: {
    type: String,
    required: false,
    default: 'text',
    validator(type) {
      return [
        'date',
        'datetime-local',
        'email',
        'month',
        'number',
        'password',
        'tel',
        'text',
        'time',
        'url',
        'week',
      ].includes(type)
    },
  },
  text: {
    type: String,
    required: false,
    default: '',
  },
  maxLength: {
    type: Number,
    required: false,
    default: -1,
  },
  disabled: {
    type: Boolean,
    required: false,
    default: false,
  },
  error: {
    type: Boolean,
    required: false,
    default: false,
  },
})
const emit = defineEmits(['update:text'])

const computedText = computed({
  get() {
    return props.text
  },
  set(value) {
    if (props.disabled) return

    emit('update:text', value)
  },
})
</script>

<template>
  <input
    v-model="computedText"
    :class="{ error }"
    :disabled="disabled"
    :placeholder="placeholder"
    :type="type"
    :maxlength="maxLength"
  />
</template>

<style lang="scss" scoped>
input {
  all: unset;
  display: flex;
  align-items: center;
  height: 2.5rem;
  padding: 0 1rem;
  border: var(--default-border);
  border-color: var(--white-500);
  border-radius: var(--large-radius);
  font-size: var(--default-font-size);
  font-weight: 600;
  color: var(--layer-0-color);
  transition-duration: var(--fast-transition-duration);

  &:focus {
    border-color: var(--white-900);
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  &::placeholder {
    opacity: 0.5;
    color: var(--layer-0-color);
  }

  @include medium-screen {
    height: 2.75rem;
  }

  &.error {
    animation: error var(--fast-transition-duration) linear;
  }
}

@keyframes error {
  from {
    transform: translateX(0);
  }

  25% {
    transform: translateX(-0.75rem);
  }

  50% {
    transform: translateX(0.5rem);
  }

  75% {
    transform: translateX(-0.25rem);
  }

  to {
    transform: translateX(0);
  }
}
</style>
