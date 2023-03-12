export const useConstantsStore = definePiniaStore('constants', () => {
  const GLOBAL_API_ERRORS = {
    // Minimum / Maximum number of ... reached
    emailTooLong: 30001,
    passwordTooShort: 30002,
    passwordTooLong: 30003,
    usernameTooShort: 30004,
    usernameTooLong: 30005,

    // Invalid payload or something else
    emailInvalid: 40001,
    usernameInvalidOrTaken: 40002,
  }

  const GLOBAL_WS_ERRORS = {
    // Minimum / Maximum number of ... reached
    greenhousesTooMany: 30001,
    greenhouseNameTooShort: 30002,
    greenhouseNameTooLong: 30003,
    greenhouseTokenTooShort: 30004,
    greenhouseTokenTooLong: 30005,
    emailTooLong: 30006,
    passwordTooShort: 30007,
    passwordTooLong: 30008,
    usernameTooShort: 30009,
    usernameTooLong: 30010,
    deviceNameTooShort: 30011,
    deviceNameTooLong: 30012,
    deviceRecordDataTooSmall: 30013,
    deviceRecordDataTooBig: 30014,
    tooLongAgo: 30015,
    futureTime: 30016,

    // Invalid body or something else
    invalidRequestField: 40001,
    greenhouseTokenTaken: 40002,
    emailInvalid: 40003,
    incorrectPassword: 40004,
    usernameInvalidOrTaken: 40005,
    invalidDeviceState: 40006,
    deviceIsNotSensor: 40007,
    deviceIsNotController: 40008,
  }

  const GLOBAL_WS_CLOSE_ERRORS = {
    unknown: 4000,
    opcode: 4001,
    invalidPayload: 4002,
    notAuthenticated: 4003,
    authenticationFailed: 4004,
    alreadyAuthenticated: 4005,
  }

  const GLOBAL_WS_OPCODES = {
    dispatch: 0,
    heartBeat: 1,
    request: 2,
    response: 3,
    error: 4,
    authorize: 5,
    subscribe: 6,
  }

  const GLOBAL_WS_EVENTS = {
    userUpdate: 'user_update',
    userMeUpdate: 'user_me_update',
    greenhouseUpdate: 'greenhouse_update',
    greenhouseCreate: 'greenhouse_create',
    deviceUpdate: 'device_update',
    deviceRecordsUpdate: 'device_records_update',
    deviceRecordsAverageUpdate: 'device_records_average_update',
  }

  const USER_THEMES = {
    auto: 0,
    light: 1,
    dark: 2,
  }

  const DEVICE_KINDS = {
    humiditySensor: 0,
    soilMoistureSensor: 1,
    temperatureSensor: 2,
    humidificationController: 3,
    irrigationController: 4,
    windowsController: 5,
  }

  const DEVICE_RECORDS_TIMESTAMP_RANGES = {
    today: 0,
    week: 1,
    month: 2,
    lastMonth: 3,
    monthBeforeLast: 4,
    lastThreeMoths: 5,
  }

  function parseGlobalApiErrorCode(code) {
    return Object.keys(GLOBAL_API_ERRORS).find(
      key => GLOBAL_API_ERRORS[key] === code
    )
  }

  function parseGlobalWsErrorCode(code) {
    return Object.keys(GLOBAL_WS_ERRORS).find(
      key => GLOBAL_WS_ERRORS[key] === code
    )
  }

  function parseGlobalWsCloseErrorCode(code) {
    return Object.keys(GLOBAL_WS_CLOSE_ERRORS).find(
      key => GLOBAL_WS_CLOSE_ERRORS[key] === code
    )
  }

  function parseGlobalWsOpcodeId(code) {
    return Object.keys(GLOBAL_WS_OPCODES).find(
      key => GLOBAL_WS_OPCODES[key] === code
    )
  }

  function parseGlobalWsEventName(code) {
    return Object.keys(GLOBAL_WS_EVENTS).find(
      key => GLOBAL_WS_EVENTS[key] === code
    )
  }

  function parseUserTheme(id) {
    return Object.keys(USER_THEMES).find(key => USER_THEMES[key] === id)
  }

  function parseDeviceKind(id) {
    return Object.keys(DEVICE_KINDS).find(key => DEVICE_KINDS[key] === id)
  }

  function parseDeviceRecordsTimestampRanges(range) {
    return Object.keys(DEVICE_RECORDS_TIMESTAMP_RANGES).find(
      key => DEVICE_RECORDS_TIMESTAMP_RANGES[key] === range
    )
  }

  return {
    GLOBAL_API_ERRORS,
    GLOBAL_WS_ERRORS,
    GLOBAL_WS_CLOSE_ERRORS,
    GLOBAL_WS_OPCODES,
    GLOBAL_WS_EVENTS,
    USER_THEMES,
    DEVICE_KINDS,
    DEVICE_RECORDS_TIMESTAMP_RANGES,
    parseGlobalApiErrorCode,
    parseGlobalWsErrorCode,
    parseGlobalWsCloseErrorCode,
    parseGlobalWsOpcodeId,
    parseGlobalWsEventName,
    parseUserTheme,
    parseDeviceKind,
    parseDeviceRecordsTimestampRanges,
  }
})
