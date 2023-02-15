export default defineNuxtPlugin(() => {
  const config = useRuntimeConfig()
  const user = useUserStore()

  return {
    provide: {
      authorizedFetch: async (path, options = {}) => {
        if (user.token === undefined || user.token === null) {
          user.setToken(localStorage.getItem('token'))
        }

        options = {
          ...options,
          headers: {
            ...options.headers,
            authorization: user.token,
            ['content-type']: 'application/json',
          },
        }

        let response = await fetch(`${config.globalApiPath}${path}`, options)
        let sessionToken = response.headers.get('x-set-session-token')

        if (sessionToken) {
          user.setToken(sessionToken)
        }

        return response
      },
    },
  }
})
