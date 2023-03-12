export const useDataStore = definePiniaStore('data', () => {
  const greenhouses = ref({})
  const devices = ref({})
  const deviceRecordsQuantities = ref({})
  const deviceRecordsAverage = ref({})

  function setGreenhouse(greenhouse) {
    greenhouses.value[greenhouse.id] = greenhouse
  }

  function setDevice(device) {
    devices.value[device.id] = device
  }

  function setDeviceRecordsQuantity(data) {
    deviceRecordsQuantities.value[data['device_id']] = data
  }

  function setDeviceRecordsAverage(data) {
    if (deviceRecordsAverage.value[data.range] === undefined)
      deviceRecordsAverage.value[data.range] = {}

    deviceRecordsAverage.value[data.range][data['device_id']] = data
  }

  function deleteAll() {
    greenhouses.value = {}
    devices.value = {}
    deviceRecordsQuantities.value = {}
    deviceRecordsAverage.value = {}
  }

  return {
    greenhouses,
    devices,
    deviceRecordsQuantities,
    deviceRecordsAverage,
    setGreenhouse,
    setDevice,
    setDeviceRecordsQuantity,
    setDeviceRecordsAverage,
    deleteAll,
  }
})
