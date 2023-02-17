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

  function setToken(value) {
    localStorage.setItem('token', value)
    token.value = value
  }

  function setIsLoggedIn(state) {
    isLoggedIn.value = state
  }

  function login(authedId, authedEmail, authedUsername, authedCreatedAt) {
    isLoggedInCookie.value = JSON.stringify(true)

    isLoggedIn.value = true
    id.value = authedEmail
    email.value = authedEmail
    username.value = authedUsername
    createdAt.value = authedCreatedAt
  }

  function logout() {
    isLoggedInCookie.value = JSON.stringify(false)

    isLoggedIn.value = false
    email.value = ''
    username.value = ''

    system.webSocket.close()
  }

  return {
    token,
    isLoggedIn,
    id,
    email,
    username,
    createdAt,
    setToken,
    setIsLoggedIn,
    login,
    logout,
  }
})
