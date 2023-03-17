export const useDataStore = definePiniaStore('data', () => {
  const greenhouses = ref({})
  const devices = ref({})
  const deviceRecordsQuantities = ref({})
  const deviceRecordsAverage = ref({})

  function setGreenhouse(greenhouse) {
    greenhouses.value[greenhouse.id] = greenhouse
  }

  function deleteGreenhouse(id) {
    delete greenhouses.value[id]

    Object.values(devices.value)
      .filter(device => device['greenhouse_id'] === id)
      .map(device => {
        delete devices[device.id]
        delete deviceRecordsQuantities.value[device.id]

        // TODO: Deletion in `deviceRecordsAverage`
        // Object.values(deviceRecordsAverage.value)
        //   .map(range => Object.values(range)
        //     .filter(record => record['device_id'] === device.id)
        //     .map(record => delete deviceRecordsAverage[range][device.id]))
      })
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
    deleteGreenhouse,
    setDevice,
    setDeviceRecordsQuantity,
    setDeviceRecordsAverage,
    deleteAll,
  }
})
