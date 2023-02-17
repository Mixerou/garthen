export const useSystemStore = definePiniaStore('system', () => {
  const modals = ref([])
  const webSocket = ref(null)
  const webSocketNextMessageId = ref(0)
  const isWebSocketAuthorized = ref(false)
  const isEetfInitialized = ref(false)

  function registerModal(id, priority = null) {
    modals.value.push(id)
  }

  function unregisterActiveModal() {
    modals.value.pop()
  }

  function openWebSocketConnection(uri) {
    webSocket.value = new WebSocket(uri)
    webSocket.value.binaryType = 'arraybuffer'
  }

  function getWebSocketMessageId() {
    return webSocketNextMessageId.value++ - 1
  }

  function setIsWebSocketAuthorized(state) {
    isWebSocketAuthorized.value = state
  }

  function setIsEetfInitialized(state) {
    isEetfInitialized.value = state
  }

  return {
    modals,
    webSocket,
    isWebSocketAuthorized,
    isEetfInitialized,
    registerModal,
    unregisterActiveModal,
    openWebSocketConnection,
    getWebSocketMessageId,
    setIsWebSocketAuthorized,
    setIsEetfInitialized,
  }
})
