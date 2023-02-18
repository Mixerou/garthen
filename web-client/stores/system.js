export const useSystemStore = definePiniaStore('system', () => {
  const modals = ref([])
  const isAppLayout = ref(false)
  const isNavbarFolded = ref(false)
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

  function setIsNavbarFolded(state) {
    isNavbarFolded.value = state
  }

  function setIsAppLayout(state) {
    isAppLayout.value = state
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
    isAppLayout,
    isNavbarFolded,
    webSocket,
    isWebSocketAuthorized,
    isEetfInitialized,
    registerModal,
    unregisterActiveModal,
    setIsAppLayout,
    setIsNavbarFolded,
    openWebSocketConnection,
    getWebSocketMessageId,
    setIsWebSocketAuthorized,
    setIsEetfInitialized,
  }
})
