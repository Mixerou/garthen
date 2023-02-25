export const useUserStore = definePiniaStore('user', () => {
  const isLoggedInCookie = useCookie('is_logged_in', {
    expires: new Date(Date.now() * 2),
  })
  const system = useSystemStore()

  const token = ref(null)
  const isLoggedIn = ref(!!JSON.parse(isLoggedInCookie.value || 'false'))
  const id = ref(0)
  const email = ref('')
  const username = ref('')
  const createdAt = ref(0)
  const locale = ref('')
  const theme = ref(0)
  const greenhousesCount = ref(0)

  function setToken(value) {
    if (value === null || value === 'null') {
      isLoggedIn.value = false

      return
    }

    localStorage.setItem('token', value)
    token.value = value
  }

  function setIsLoggedIn(state) {
    isLoggedIn.value = state
  }

  function login(
    authedId,
    authedEmail,
    authedUsername,
    authedCreatedAt,
    authedLocale,
    authedTheme,
    authedGreenhousesCount
  ) {
    isLoggedInCookie.value = JSON.stringify(true)

    system.setLocale(authedLocale)
    system.setTheme(authedTheme)

    isLoggedIn.value = true
    id.value = authedId
    email.value = authedEmail
    username.value = authedUsername
    createdAt.value = authedCreatedAt
    locale.value = authedLocale
    theme.value = authedTheme
    greenhousesCount.value = authedGreenhousesCount
  }

  function logout() {
    isLoggedInCookie.value = JSON.stringify(false)

    isLoggedIn.value = false
    id.value = 0
    email.value = ''
    username.value = ''
    createdAt.value = 0
    locale.value = ''
    theme.value = 0
    greenhousesCount.value = 0

    system.webSocket.close()
  }

  return {
    token,
    isLoggedIn,
    id,
    email,
    username,
    createdAt,
    locale,
    theme,
    greenhousesCount,
    setToken,
    setIsLoggedIn,
    login,
    logout,
  }
})
