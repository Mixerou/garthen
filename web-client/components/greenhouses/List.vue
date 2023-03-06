<script setup>
import IconPsychiatry from '@/assets/icons/psychiatry.svg?skipsvgo'

const props = defineProps({
  animationsDisabled: {
    type: Boolean,
    required: false,
    default: false,
  },
  excludedGreenhouses: {
    type: Array,
    required: false,
    default: () => [],
  },
})

const dataStore = useDataStore()
const user = useUserStore()

const greenhouses = computed(() => {
  let result = {}

  for (const greenhouse of Object.values(dataStore.greenhouses)) {
    if (!props.excludedGreenhouses.includes(greenhouse.id)) {
      result[greenhouse.id] = greenhouse
    }
  }

  return result
})
</script>

<template>
  <div v-if="user.greenhousesCount !== 0" class="greenhouses">
    <TransitionGroup
      enter-active-class="transition"
      enter-from-class="hide"
      leave-active-class="transition"
      leave-to-class="hide"
    >
      <NuxtLink
        v-for="greenhouse in greenhouses"
        :key="`greenhouse-${greenhouse.id}`"
        class="greenhouse"
        :class="{
          ['disable-transition-animation']: animationsDisabled,
        }"
        :to="`/greenhouses/${greenhouse.id}`"
      >
        <GarthenButton>
          <IconPsychiatry class="icon" />
          <span>{{ greenhouse.name }}</span>
        </GarthenButton>
      </NuxtLink>
    </TransitionGroup>
  </div>
</template>

<style lang="scss" scoped>
.greenhouses {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  width: 16rem;
  max-height: 20rem;
  padding: 0.25rem 0.5rem;
  border-radius: var(--large-radius);
  background: var(--primary);
  overflow-y: auto;
  transition: var(--default-transition);

  @include medium-screen {
    gap: 0.5rem;
    width: 20rem;
    padding: 0.5rem 1rem;
  }

  &.hide {
    max-height: 0;
    margin-bottom: calc(-1rem - 0.25rem * 2);
    opacity: 0;

    @include medium-screen {
      margin-bottom: calc(-1rem - 0.5rem * 2);
    }
  }

  .greenhouse {
    width: 100%;
    border-radius: var(--large-radius);
    text-decoration: none;

    &.hide:not(.disable-transition-animation) {
      margin-top: calc(-2.5rem - 0.25rem);
      opacity: 0;

      @include medium-screen {
        margin-top: calc(-2.75rem - 0.5rem);
      }
    }

    &.transition {
      transition: var(--default-transition);
    }

    button {
      width: calc(100% - 2rem - 1rem * 2);

      @include medium-screen {
        width: calc(100% - 1.5rem - 1rem * 2);
      }
    }

    .icon {
      width: 1.5rem;

      @include medium-screen {
        width: 1.75rem;
      }
    }
  }
}
</style>
