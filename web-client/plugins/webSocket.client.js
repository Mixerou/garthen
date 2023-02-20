import init, { pack, unpack } from 'serde-eetf'
import serdeEetfWasmUrl from '@/node_modules/serde-eetf/serde_eetf_bg.wasm?url'

export default defineNuxtPlugin(plugin => {
  const config = useRuntimeConfig()
  const constants = useConstantsStore()
  const dataStore = useDataStore()
  const system = useSystemStore()
  const user = useUserStore()

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
          system.setIsWebSocketAuthorized(false)

          if (
            event.code === constants.GLOBAL_WS_CLOSE_ERRORS.authenticationFailed
          ) {
            user.setIsLoggedIn(false)

            return
          }

          if (event.code === 1005) return

          setTimeout(() => plugin.$wsOpenConnection(), 1e3)
        }

        system.webSocket.onmessage = message => {
          message = JSON.parse(unpack(new Uint8Array(message.data)))

          if (message.o !== constants.GLOBAL_WS_OPCODES.dispatch) return

          const event = message.e[0]
          const data = message.d

          if (event === constants.GLOBAL_WS_EVENTS.userUpdate) {
            // TODO: Add logic as needed
          } else if (event === constants.GLOBAL_WS_EVENTS.userMeUpdate) {
            user.login(
              data.id,
              data.email,
              data.username,
              data['created_at'],
              data.greenhouses
            )
          } else if (event === constants.GLOBAL_WS_EVENTS.greenhouseUpdate) {
            dataStore.setGreenhouse(data)
          } else if (event === constants.GLOBAL_WS_EVENTS.greenhouseCreate) {
            // TODO: dataStore.setGreenhouse(data)

            plugin.$wsSend({
              o: constants.GLOBAL_WS_OPCODES.subscribe,
              r: 'greenhouse',
              d: {
                id: data.id,
              },
            })
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
                  token: user.token,
                },
              }

              if (!system.isEetfInitialized) {
                await init(serdeEetfWasmUrl)
                system.setIsEetfInitialized(true)
              }

              system.webSocket.send(pack(JSON.stringify(data)).buffer, {
                binary: true,
              })

              system.webSocket.addEventListener(
                'message',
                () => {
                  heartbeating = setInterval(() => {
                    plugin.$wsSend({ o: constants.GLOBAL_WS_OPCODES.heartBeat })
                  }, 30e3)

                  system.setIsWebSocketAuthorized(true)
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
          !system.isWebSocketAuthorized ||
          system.webSocket.readyState !== system.webSocket.OPEN
        ) {
          system.webSocket.addEventListener(
            'message',
            () => {
              system.webSocket.send(pack(JSON.stringify(data)).buffer, {
                binary: true,
              })
            },
            { once: true }
          )

          return
        }

        system.webSocket.send(pack(JSON.stringify(data)).buffer, {
          binary: true,
        })
      },
      wsSendAndWait: async data => {
        data.i = system.getWebSocketMessageId()

        return new Promise(resolve => {
          if (
            !system.isWebSocketAuthorized ||
            system.webSocket.readyState !== system.webSocket.OPEN
          ) {
            system.webSocket.addEventListener(
              'message',
              () => {
                system.webSocket.send(pack(JSON.stringify(data)).buffer, {
                  binary: true,
                })

                system.webSocket.addEventListener(
                  'message',
                  function handler(message) {
                    message = JSON.parse(unpack(new Uint8Array(message.data)))

                    if (data.i === message.i) {
                      this.removeEventListener('message', handler)

                      resolve(message)
                    }
                  }
                )
              },
              { once: true }
            )

            return
          }

          system.webSocket.send(pack(JSON.stringify(data)).buffer, {
            binary: true,
          })

          system.webSocket.addEventListener(
            'message',
            function handler(message) {
              message = JSON.parse(unpack(new Uint8Array(message.data)))

              if (data.i === message.i) {
                this.removeEventListener('message', handler)

                resolve(message)
              }
            }
          )
        })
      },
    },
  }
})
