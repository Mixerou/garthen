export const useUserStore = definePiniaStore('user', () => {
  // TODO: Remove
  const devUserCredentialsCookie = useCookie('dev_user_credentials', {
    expires: new Date(Date.now() * 2),
  })
  const isLoggedInCookie = useCookie('is_logged_in', {
    expires: new Date(Date.now() * 2),
  })

  const isLoggedIn = ref(!!JSON.parse(isLoggedInCookie.value || 'false'))
  const email = ref('')
  const username = ref('')

  function login(authedEmail, authedUsername) {
    isLoggedInCookie.value = JSON.stringify(true)

    isLoggedIn.value = true
    email.value = authedEmail
    username.value = authedUsername

    // TODO: Remove
    devUserCredentialsCookie.value = { email: email, username: username }
  }

  function logout() {
    isLoggedInCookie.value = JSON.stringify(false)

    isLoggedIn.value = false
    email.value = ''
    username.value = ''
  }

  return {
    isLoggedIn,
    email,
    username,
    login,
    logout,
  }
})
