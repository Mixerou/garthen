export const useSystemStore = definePiniaStore('system', () => {
  const modals = ref([])

  function registerModal(id, priority = null) {
    modals.value.push(id)
  }

  function unregisterActiveModal() {
    modals.value.pop()
  }

  return {
    modals,
    registerModal,
    unregisterActiveModal,
  }
})
