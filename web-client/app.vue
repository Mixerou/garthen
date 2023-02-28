<script setup>
const { $wsOpenConnection, $wsSubscribe } = useNuxtApp()
const constants = useConstantsStore()
const system = useSystemStore()
const user = useUserStore()
const { locale, locales, t } = useI18n()
const localeCookie = useCookie('locale', {
  expires: new Date(Date.now() * 2),
})
const themeCookie = useCookie('theme', {
  expires: new Date(Date.now() * 2),
})

const getClientLocale = () => {
  if (
    localeCookie.value === undefined ||
    localeCookie.value === null ||
    localeCookie.value.constructor !== String
  ) {
    localeCookie.value = useBrowserLocale()
  }

  let clientLocale = localeCookie.value.includes('-')
    ? locales.value.find(locale => locale.code === localeCookie.value)
    : locales.value.find(locale => locale.code.startsWith(localeCookie.value))

  if (clientLocale === undefined)
    clientLocale = locales.value.find(element => element.code === locale.value)
  if (localeCookie.value !== clientLocale.code)
    localeCookie.value = clientLocale.code

  return clientLocale
}

const clientLocale = getClientLocale()

system.setLocale(clientLocale.code)
system.setTheme(
  constants.USER_THEMES[constants.parseUserTheme(themeCookie.value) || 'light']
)

watchEffect(() => {
  locale.value = system.locale

  // TODO: Auto theme
  const theme =
    system.theme === constants.USER_THEMES.light ||
    system.theme === constants.USER_THEMES.dark
      ? constants.parseUserTheme(system.theme)
      : 'light'

  useHead({
    titleTemplate: title => (title ? `${title} - Garthen` : 'Garthen'),
    htmlAttrs: {
      lang: system.locale,
    },
    bodyAttrs: {
      ['data-theme']: theme,
    },
    meta: [
      {
        name: 'description',
        content: t('garthen.description.short'),
        lang: system.locale,
      },
    ],
  })
})

onMounted(async () => {
  user.setToken(localStorage.getItem('token'))

  if (user.isLoggedIn && user.token !== null) {
    $wsOpenConnection()
    $wsSubscribe('user/me', {}, true)
  }
})
</script>

<template>
  <div id="modals" />
  <NuxtLayout class="layout">
    <NuxtPage />
  </NuxtLayout>
</template>

<style lang="scss">
* {
  margin: 0;
  outline: none;
  -webkit-tap-highlight-color: transparent;
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
  font-family: 'Nunito Sans', sans-serif;
}

body {
  display: flex;
  justify-content: center;
  width: 100vw;
  height: 100vh;
  overflow-x: hidden;
  scrollbar-width: none;
  background: var(--layer-0-background);
  color: var(--layer-0-color);
  transition: var(--default-transition);
  transition-property: background-color, color;

  &::-webkit-scrollbar {
    display: none;
  }
}

h1 {
  line-height: var(--4xl-line-height);
  font-size: var(--4xl-font-size);
}

h2 {
  line-height: var(--3xl-line-height);
  font-size: var(--3xl-font-size);
}

h3 {
  line-height: var(--2xl-line-height);
  font-size: var(--2xl-font-size);
}

h4 {
  line-height: var(--xl-line-height);
  font-size: var(--xl-font-size);
}

h5 {
  line-height: var(--large-line-height);
  font-size: var(--large-font-size);
}

h6 {
  line-height: var(--medium-line-height);
  font-size: var(--medium-font-size);
}

.layout {
  position: absolute;
  top: 0;
  left: 0;

  &.layout-enter-active,
  &.layout-leave-active {
    transition: var(--default-transition);
  }

  &.layout-enter-from {
    opacity: 0;
    transform: scale(0.95);
  }

  &.layout-leave-to {
    opacity: 0;
    transform: scale(1.05);
  }
}
</style>
