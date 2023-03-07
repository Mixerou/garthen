<script setup>
const props = defineProps({
  editMenuOpened: {
    type: Boolean,
    required: true,
  },
  name: {
    type: String,
    required: true,
  },
  defaultName: {
    type: String,
    required: false,
    default: '',
  },
})
const emit = defineEmits(['update:name'])

const computedName = computed({
  get() {
    return props.name
  },
  set(value) {
    emit('update:name', value)
  },
})
</script>

<template>
  <div class="menu">
    <TransitionGroup enter-from-class="hide" leave-to-class="hide">
      <GarthenInput
        v-if="editMenuOpened"
        v-model:text="computedName"
        class="name-input"
        :placeholder="defaultName"
        max-length="24"
      />
    </TransitionGroup>
  </div>
</template>

<style lang="scss" scoped>
.menu {
  position: absolute;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 0.5rem;
  top: 0.625rem;
  right: 1rem;

  .name-input {
    width: calc(100% - 5rem);

    @include medium-screen {
      width: calc(100% - 4.125rem);
    }
  }

  .hide {
    opacity: 0;

    &.name-input {
      transform: translateX(100%);
    }
  }
}
</style>
