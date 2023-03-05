import svgLoader from 'vite-svg-loader'

export default defineNuxtConfig({
  app: {
    head: {
      viewport: 'width=device-width, initial-scale=1, maximum-scale=1',
      charset: 'utf-8',
      meta: [{ name: 'format-detection', content: 'telephone=no' }],
      link: [
        { rel: 'dns-prefetch', href: 'https://fonts.gstatic.com/' },
        { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
        {
          rel: 'preconnect',
          href: 'https://fonts.gstatic.com',
          crossorigin: true,
        },
        {
          rel: 'stylesheet',
          href: 'https://fonts.googleapis.com/css2?family=Nunito+Sans:wght@400;600;700;900&display=swap',
        },
      ],
    },
    layoutTransition: { name: 'layout' },
    pageTransition: { name: 'page', mode: 'out-in' },
  },
  css: ['@/assets/styles/_colors.scss', '@/assets/styles/_variables.scss'],
  modules: [
    [
      '@nuxtjs/i18n',
      {
        skipSettingLocaleOnNavigate: true,
        strategy: 'no_prefix',
        locales: [
          {
            code: 'en-GB',
            file: 'en-GB.json',
            iso: 'en-GB',
            name: 'English, UK',
          },
          {
            code: 'ru-RU',
            file: 'ru-RU.json',
            iso: 'ru-RU',
            name: 'Русский',
          },
        ],
        langDir: 'assets/locales',
        defaultLocale: 'en-GB',
        detectBrowserLanguage: {
          useCookie: false,
        },
        vueI18n: {
          globalInjection: true,
          legacy: false,
          locale: 'en-GB',
          fallbackLocale: 'en-GB',
        },
      },
    ],
    [
      '@pinia/nuxt',
      {
        autoImports: [['defineStore', 'definePiniaStore']],
      },
    ],
  ],
  imports: {
    dirs: ['./stores'],
  },
  vite: {
    css: {
      preprocessorOptions: {
        scss: {
          additionalData:
            '@import "@/assets/styles/mixins/_screens.scss"; @import "@/assets/styles/mixins/_widths.scss";',
        },
      },
    },
    plugins: [svgLoader()],
  },
  runtimeConfig: {
    public: {
      globalApiPath: '',
      globalWsUri: '',
    },
  },
  telemetry: false,
})
