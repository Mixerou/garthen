export const useDataStore = definePiniaStore('data', () => {
  const greenhouses = ref({})

  function setGreenhouse(greenhouse) {
    greenhouses.value[greenhouse.id] = greenhouse
  }

  function deleteData() {
    greenhouses.value = {}
  }

  return {
    greenhouses,
    setGreenhouse,
    deleteData,
  }
})