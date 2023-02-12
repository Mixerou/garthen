<script setup>
const system = useSystemStore()
const user = useUserStore()
const { locale, locales, t } = useI18n()
const localeCookie = useCookie('locale', {
  expires: new Date(Date.now() * 2),
})
// TODO: Remove
const devUserCredentialsCookie = useCookie('dev_user_credentials', {
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

locale.value = clientLocale.code

useHead({
  titleTemplate: title => (title ? `${title} - Garthen` : 'Garthen'),
  htmlAttrs: {
    lang: clientLocale.iso,
  },
  meta: [
    {
      name: 'description',
      content: t('garthen.description.short'),
      lang: clientLocale.iso,
    },
  ],
})

onMounted(() => {
  if (user.isLoggedIn) {
    setTimeout(() => {
      // TODO: Retrieve it from the server
      const credentials = devUserCredentialsCookie.value

      user.login(credentials.email, credentials.username)
    }, 1500)
  }
})
</script>

<template>
  <div id="app">
    <div id="modals" />
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </div>
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
  background: var(--layer-0-background);
  color: var(--layer-0-color);
  transition: var(--default-transition);
  transition-property: background-color, color;
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
</style>
