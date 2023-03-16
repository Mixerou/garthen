import init, { pack, unpack } from 'serde-eetf'
import serdeEetfWasmUrl from '@/node_modules/serde-eetf/serde_eetf_bg.wasm?url'

export default defineNuxtPlugin(plugin => {
  const config = useRuntimeConfig()
  const route = useRoute()
  const router = useRouter()
  const constants = useConstantsStore()
  const dataStore = useDataStore()
  const system = useSystemStore()
  const user = useUserStore()

  const isBigNumber = number => !Number.isSafeInteger(+number)

  const enquoteBigNumber = jsonString =>
    jsonString.replaceAll(
      /([:\s\[,]*)(\d+)([\s,\]]*)/g,
      (matchingSubstring, prefix, bigNumber, suffix) =>
        isBigNumber(bigNumber)
          ? `${prefix}"${bigNumber}"${suffix}`
          : matchingSubstring
    )

  const parseJson = jsonString =>
    JSON.parse(
      enquoteBigNumber(jsonString.replaceAll('"nil"', 'null')),
      (key, value) =>
        !isNaN(value) && isBigNumber(value) && !/[.]/.test(String(value))
          ? BigInt(value)
          : value
    )

  const stringifyToJson = data => {
    if (data !== undefined) {
      let intCount = 0
      let repCount = 0

      const json = JSON.stringify(data, (_, value) => {
        if (typeof value === 'bigint') {
          intCount++

          return `${value}#bigint`
        }

        return value
      })

      const result = json.replace(/"(-?\d+)#bigint"/g, (_, value) => {
        repCount++

        return value
      })

      if (repCount > intCount) {
        throw new Error(`BigInt serialization conflict with a string value.`)
      }

      return result
    }
  }

  return {
    provide: {
      wsOpenConnection: async () => {
        system.openWebSocketConnection(config.globalWsUri)

        let heartbeating

        system.webSocket.onerror = event => {
          clearInterval(heartbeating)
          system.setIsWebSocketAuthorized(false)
        }

        system.webSocket.onclose = event => {
          clearInterval(heartbeating)
          system.setIsWebSocketConnected(false)
          system.setIsWebSocketAuthorized(false)
          dataStore.deleteAll()

          if (
            event.code === constants.GLOBAL_WS_CLOSE_ERRORS.authenticationFailed
          ) {
            system.deleteWebSocketSubscriptions()
            user.setIsLoggedIn(false)

            return
          }

          if (event.code === 1005) {
            system.deleteWebSocketSubscriptions()
            return
          }

          setTimeout(() => plugin.$wsOpenConnection(), 1e3)
        }

        system.webSocket.onmessage = message => {
          message = parseJson(unpack(new Uint8Array(message.data)))

          if (message.o !== constants.GLOBAL_WS_OPCODES.dispatch) return

          const event = message.e.n
          const data = message.d

          if (event === constants.GLOBAL_WS_EVENTS.userUpdate) {
            // TODO: Add logic as needed
          } else if (event === constants.GLOBAL_WS_EVENTS.userMeUpdate) {
            user.login(
              data.id,
              data.email,
              data.username,
              data['created_at'],
              data.locale,
              data.theme,
              data.greenhouses
            )
          } else if (event === constants.GLOBAL_WS_EVENTS.greenhouseUpdate) {
            dataStore.setGreenhouse(data)
          } else if (event === constants.GLOBAL_WS_EVENTS.greenhouseCreate) {
            // TODO: dataStore.setGreenhouse(data)

            plugin.$wsSubscribe(
              'greenhouse',
              { a: 'subscribe_to_greenhouse_update', id: data.id },
              true,
              false
            )
          } else if (event === constants.GLOBAL_WS_EVENTS.greenhouseDelete) {
            try {
              if (BigInt(route.params.greenhouseId) === data.id) {
                router.push('/greenhouses')
              }
            } catch {}

            dataStore.deleteGreenhouse(data.id)
          } else if (event === constants.GLOBAL_WS_EVENTS.deviceUpdate) {
            dataStore.setDevice(data)
          } else if (event === constants.GLOBAL_WS_EVENTS.deviceRecordsUpdate) {
            dataStore.setDeviceRecordsQuantity(data)
          } else if (
            event === constants.GLOBAL_WS_EVENTS.deviceRecordsAverageUpdate
          ) {
            dataStore.setDeviceRecordsAverage(data)
          }
        }

        return new Promise(resolve => {
          system.webSocket.addEventListener(
            'open',
            async () => {
              const data = {
                i: system.getWebSocketMessageId(),
                o: constants.GLOBAL_WS_OPCODES.authorize,
                d: {
                  a: 'authorize',
                  token: user.token,
                },
              }

              if (!system.isEetfInitialized) {
                await init(serdeEetfWasmUrl)
                system.setIsEetfInitialized(true)
              }

              system.webSocket.send(pack(stringifyToJson(data)).buffer, {
                binary: true,
              })

              for (const subscription of system.webSocketSubscriptions) {
                plugin.$wsSubscribe(
                  subscription.request,
                  subscription.data,
                  false,
                  false
                )
              }

              system.webSocket.addEventListener(
                'message',
                () => {
                  heartbeating = setInterval(() => {
                    plugin.$wsSend({ o: constants.GLOBAL_WS_OPCODES.heartBeat })
                  }, 30e3)

                  system.setIsWebSocketAuthorized(true)
                  system.setIsWebSocketConnected(true)
                  resolve()
                },
                { once: true }
              )
            },
            { once: true }
          )
        })
      },
      wsSend: data => {
        data.i = system.getWebSocketMessageId()

        if (
          data.d === undefined ||
          Object.keys(data.d).length === 0 ||
          data.d.a === undefined
        ) {
          delete data.d
        }

        if (
          !system.isWebSocketAuthorized ||
          !system.isWebSocketConnected ||
          system.webSocket.readyState !== system.webSocket.OPEN
        ) {
          try {
            system.webSocket.addEventListener(
              'message',
              () => {
                system.webSocket.send(pack(stringifyToJson(data)).buffer, {
                  binary: true,
                })
              },
              { once: true }
            )
          } catch {}

          return
        }

        system.webSocket.send(pack(stringifyToJson(data)).buffer, {
          binary: true,
        })
      },
      wsSendAndWait: async data => {
        data.i = system.getWebSocketMessageId()

        if (
          data.d === undefined ||
          Object.keys(data.d).length === 0 ||
          data.d.a === undefined
        ) {
          delete data.d
        }

        return new Promise(resolve => {
          if (
            !system.isWebSocketAuthorized ||
            !system.isWebSocketConnected ||
            system.webSocket.readyState !== system.webSocket.OPEN
          ) {
            try {
              system.webSocket.addEventListener(
                'message',
                () => {
                  system.webSocket.send(pack(stringifyToJson(data)).buffer, {
                    binary: true,
                  })

                  system.webSocket.addEventListener(
                    'message',
                    function handler(message) {
                      message = parseJson(unpack(new Uint8Array(message.data)))

                      if (data.i === message.i) {
                        this.removeEventListener('message', handler)

                        resolve(message)
                      }
                    }
                  )
                },
                { once: true }
              )
            } catch {}

            return
          }

          system.webSocket.send(pack(stringifyToJson(data)).buffer, {
            binary: true,
          })

          system.webSocket.addEventListener(
            'message',
            function handler(message) {
              message = parseJson(unpack(new Uint8Array(message.data)))

              if (data.i === message.i) {
                this.removeEventListener('message', handler)

                resolve(message)
              }
            }
          )
        })
      },
      wsSubscribe: (
        request,
        data = {},
        needToCheckExistence = false,
        needToRememberSubscription = true
      ) => {
        if (needToCheckExistence) {
          for (const subscription of system.webSocketSubscriptions.filter(
            subscription => subscription.request === request
          )) {
            if (stringifyToJson(subscription.data) === stringifyToJson(data)) {
              return
            }
          }
        }

        if (needToRememberSubscription)
          system.addWebSocketSubscription({ request, data })

        plugin.$wsSend({
          o: constants.GLOBAL_WS_OPCODES.subscribe,
          r: request,
          d: data,
        })
      },
    },
  }
})
