<script setup>
const props = defineProps({
  layer: {
    type: String,
    required: false,
    default: 'layer-0',
    validator(layer) {
      // from _variables.scss
      return [
        'layer-0',
        'layer-1',
        'primary-layer-0',
        'primary-layer-1',
      ].includes(layer)
    },
  },
  small: {
    type: Boolean,
    required: false,
    default: false,
  },
})

const firstLettersWithColor = ref('')
const lastLettersColor = ref('')

watchEffect(() => {
  if (props.layer.startsWith('layer')) {
    firstLettersWithColor.value = `var(--primary-500)`
    lastLettersColor.value = `var(--${props.layer}-color)`
  } else if (props.layer === 'primary-layer-0') {
    firstLettersWithColor.value = `var(--white-original)`
    lastLettersColor.value = `var(--white-original)`
  } else if (props.layer.startsWith('primary-layer')) {
    firstLettersWithColor.value = `var(--white-original)`
    lastLettersColor.value = `var(--primary-original-700)`
  }
})
</script>

<template>
  <NuxtLink class="logo" to="/">
    <svg
      v-if="small"
      width="60"
      height="49"
      viewBox="0 0 60 49"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        class="first-letters"
        d="M15.264 36.396C12.36 36.396 9.888 35.856 7.848 34.776C5.832 33.696 4.296 32.184 3.24 30.24C2.184 28.296 1.656 26.028 1.656 23.436C1.656 20.748 2.208 18.42 3.312 16.452C4.44 14.46 6.048 12.924 8.136 11.844C10.224 10.764 12.744 10.224 15.696 10.224C17.472 10.224 19.164 10.464 20.772 10.944C22.38 11.4 23.688 12.024 24.696 12.816L22.68 17.928C21.576 17.208 20.46 16.692 19.332 16.38C18.204 16.044 17.004 15.876 15.732 15.876C13.308 15.876 11.496 16.524 10.296 17.82C9.12 19.116 8.532 20.988 8.532 23.436C8.532 25.884 9.132 27.732 10.332 28.98C11.532 30.228 13.308 30.852 15.66 30.852C16.764 30.852 18 30.684 19.368 30.348V26.352H14.292V21.708H24.768V34.452C23.424 35.076 21.924 35.556 20.268 35.892C18.612 36.228 16.944 36.396 15.264 36.396Z"
      />
      <path
        class="last-letters"
        d="M35.1745 36V16.02H27.1105V10.62H49.8625V16.02H41.7985V36H35.1745Z"
      />
      <path class="dot" d="M51.6105 36V29.448H58.2705V36H51.6105Z" />
    </svg>

    <svg v-else viewBox="0 0 188 27" xmlns="http://www.w3.org/2000/svg">
      <path
        class="first-letters"
        d="M14.264 26.396C11.36 26.396 8.888 25.856 6.848 24.776C4.832 23.696 3.296 22.184 2.24 20.24C1.184 18.296 0.656 16.028 0.656 13.436C0.656 10.748 1.208 8.42 2.312 6.452C3.44 4.46 5.048 2.924 7.136 1.844C9.224 0.763999 11.744 0.223999 14.696 0.223999C16.472 0.223999 18.164 0.463999 19.772 0.943999C21.38 1.4 22.688 2.024 23.696 2.816L21.68 7.928C20.576 7.208 19.46 6.692 18.332 6.38C17.204 6.044 16.004 5.876 14.732 5.876C12.308 5.876 10.496 6.524 9.296 7.82C8.12 9.116 7.532 10.988 7.532 13.436C7.532 15.884 8.132 17.732 9.332 18.98C10.532 20.228 12.308 20.852 14.66 20.852C15.764 20.852 17 20.684 18.368 20.348V16.352H13.292V11.708H23.768V24.452C22.424 25.076 20.924 25.556 19.268 25.892C17.612 26.228 15.944 26.396 14.264 26.396ZM25.1385 26L37.0185 0.619998H42.2385L54.1185 26H47.4945L45.2985 20.852H33.9585L31.7625 26H25.1385ZM39.5745 7.532L36.1185 15.776H43.1025L39.6465 7.532H39.5745ZM55.6556 26V0.619998H67.8596C70.7396 0.619998 72.9596 1.328 74.5196 2.744C76.1036 4.136 76.8956 6.08 76.8956 8.576C76.8956 10.472 76.4156 12.068 75.4556 13.364C74.5196 14.636 73.1516 15.524 71.3516 16.028C72.6956 16.388 73.7756 17.312 74.5916 18.8L78.5516 26H71.2796L66.8876 17.936C66.5996 17.432 66.2276 17.072 65.7716 16.856C65.3396 16.64 64.8356 16.532 64.2596 16.532H62.2796V26H55.6556ZM62.2796 11.852H66.6716C69.2876 11.852 70.5956 10.82 70.5956 8.756C70.5956 6.716 69.2876 5.696 66.6716 5.696H62.2796V11.852Z"
      />
      <path
        class="last-letters"
        d="M87.0495 26V6.02H78.9855V0.619998H101.738V6.02H93.6735V26H87.0495ZM104.242 26V0.619998H110.866V10.412H121.342V0.619998H127.93V26H121.342V15.848H110.866V26H104.242ZM132.542 26V0.619998H150.47V5.732H138.878V10.556H149.642V15.668H138.878V20.888H150.47V26H132.542ZM154.656 26V0.619998H159.516L171.18 15.164V0.619998H177.3V26H172.476L160.812 11.456V26H154.656Z"
      />
      <path class="dot" d="M181.181 26V19.448H187.841V26H181.181Z" />
    </svg>
  </NuxtLink>
</template>

<style lang="scss" scoped>
.logo {
  transition: opacity var(--fast-transition-duration);

  &:hover {
    opacity: 0.9;
  }

  &:active {
    opacity: 0.75;
  }

  svg {
    width: 9.75rem;

    @include small-screen {
      width: 10.75rem;
    }

    @include large-screen {
      width: 11.75rem;
    }

    .first-letters,
    .dot {
      fill: v-bind(firstLettersWithColor);
      transition: var(--default-transition);
    }

    .last-letters {
      fill: v-bind(lastLettersColor);
      transition: var(--default-transition);
    }
  }
}
</style>
