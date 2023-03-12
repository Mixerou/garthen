<script setup>
const { $wsSendAndWait } = useNuxtApp()
const { locales } = useI18n()
const constants = useConstantsStore()
const system = useSystemStore()
const user = useUserStore()

const isDisabled = ref(false)

const computedLocales = computed(() => {
  let preparedLocales = []

  locales.value.map(locale =>
    preparedLocales.push({ id: locale.code, value: locale.name })
  )

  return preparedLocales
})

const update = async (theme = system.theme, locale = system.locale) => {
  isDisabled.value = true

  if (user.isLoggedIn) {
    await $wsSendAndWait({
      o: 2,
      r: 'user/me',
      m: { patch: null },
      d: {
        a: 'request_patch_user',
        email: user.email,
        username: user.username,
        locale: locale,
        theme: theme,
      },
    })
  } else {
    system.setLocale(locale)
    system.setTheme(theme)
  }

  isDisabled.value = false
}
</script>

<template>
  <footer>
    <div class="content">
      <GarthenLogo layer="primary-layer-0" />
      <div class="actions">
        <GarthenSelect
          :disabled="isDisabled"
          :items="[
            { id: constants.USER_THEMES.light, value: $t('themes.light') },
            { id: constants.USER_THEMES.dark, value: $t('themes.dark') },
            { id: constants.USER_THEMES.auto, value: $t('themes.auto') },
          ]"
          dropdown-direction="top"
          :selected="system.theme"
          @update:selected="theme => update(theme, system.locale)"
        />

        <GarthenSelect
          class="locale-select"
          :disabled="isDisabled"
          :items="computedLocales"
          dropdown-direction="top"
          :selected="system.locale"
          @update:selected="locale => update(system.theme, locale)"
        />
      </div>
    </div>
  </footer>
</template>

<style lang="scss" scoped>
footer {
  position: relative;
  width: 100%;
  height: 8rem;

  @include medium-screen {
    height: 5.5rem;
  }

  .content {
    position: absolute;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 0.25rem;
    bottom: 0.5rem;
    left: 0.5rem;
    width: calc(100vw - 5rem);
    height: 6.5rem;
    padding: 0 2rem;
    border-radius: var(--large-radius);
    background: var(--primary);
    transition: background-color var(--default-transition);

    @include medium-screen {
      flex-direction: row;
      justify-content: space-between;
      gap: 1rem;
      height: 4.5rem;
    }

    .actions {
      display: flex;
      gap: 1rem;

      &:deep(.select) {
        .selected-value {
          border-color: #ffffff66;
          color: var(--primary-layer-0-color);

          &.opened:not(.disabled) {
            border-color: #ffffff;
          }
        }

        .options {
          border-color: #ffffff;
          color: var(--primary-layer-0-color);
        }
      }

      .locale-select:deep(.options) {
        width: 8rem;
      }
    }
  }
}
</style>
