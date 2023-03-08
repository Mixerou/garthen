export const useDataStore = definePiniaStore('data', () => {
  const greenhouses = ref({})
  const devices = ref({})
  const deviceRecordsQuantities = ref({})

  function setGreenhouse(greenhouse) {
    greenhouses.value[greenhouse.id] = greenhouse
  }

  function setDevice(device) {
    devices.value[device.id] = device
  }

  function setDeviceRecordsQuantity(data) {
    deviceRecordsQuantities.value[data['device_id']] = data
  }

  function deleteData() {
    greenhouses.value = {}
  }

  return {
    greenhouses,
    devices,
    deviceRecordsQuantities,
    setGreenhouse,
    setDevice,
    setDeviceRecordsQuantity,
    deleteData,
  }
})
