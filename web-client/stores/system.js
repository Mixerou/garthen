export const useSystemStore = definePiniaStore('system', () => {
  const modals = ref([])
  const isAppLayout = ref(false)
  const isNavbarFolded = ref(false)
  const webSocket = ref(null)
  const webSocketNextMessageId = ref(0)
  const isWebSocketAuthorized = ref(false)
  const isWebSocketConnected = ref(false)
  const webSocketSubscriptions = ref([])
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

  function setIsWebSocketConnected(state) {
    isWebSocketConnected.value = state
  }

  function addWebSocketSubscription(subscription) {
    webSocketSubscriptions.value.push(subscription)
  }

  function deleteWebSocketSubscriptions() {
    webSocketSubscriptions.value = []
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
    isWebSocketConnected,
    webSocketSubscriptions,
    isEetfInitialized,
    registerModal,
    unregisterActiveModal,
    setIsAppLayout,
    setIsNavbarFolded,
    openWebSocketConnection,
    getWebSocketMessageId,
    setIsWebSocketAuthorized,
    setIsWebSocketConnected,
    addWebSocketSubscription,
    deleteWebSocketSubscriptions,
    setIsEetfInitialized,
  }
})
