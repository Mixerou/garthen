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
    authedGreenhousesCount
  ) {
    isLoggedInCookie.value = JSON.stringify(true)

    isLoggedIn.value = true
    id.value = authedEmail
    email.value = authedEmail
    username.value = authedUsername
    createdAt.value = authedCreatedAt
    greenhousesCount.value = authedGreenhousesCount
  }

  function logout() {
    isLoggedInCookie.value = JSON.stringify(false)

    isLoggedIn.value = false
    id.value = 0
    email.value = ''
    username.value = ''
    createdAt.value = 0
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
    greenhousesCount,
    setToken,
    setIsLoggedIn,
    login,
    logout,
  }
})
