export const useDataStore = definePiniaStore('data', () => {
  const greenhouses = ref({})
  const devices = ref({})

  function setGreenhouse(greenhouse) {
    greenhouses.value[greenhouse.id] = greenhouse
  }

  function setDevice(device) {
    devices.value[device.id] = device
  }

  function deleteData() {
    greenhouses.value = {}
  }

  return {
    greenhouses,
    devices,
    setGreenhouse,
    setDevice,
    deleteData,
  }
})
