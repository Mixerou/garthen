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

    // Invalid body or something else
    invalidRequestField: 40001,
    greenhouseTokenTaken: 40002,
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

  return {
    GLOBAL_API_ERRORS,
    GLOBAL_WS_ERRORS,
    GLOBAL_WS_CLOSE_ERRORS,
    GLOBAL_WS_OPCODES,
    GLOBAL_WS_EVENTS,
    parseGlobalApiErrorCode,
    parseGlobalWsErrorCode,
    parseGlobalWsCloseErrorCode,
    parseGlobalWsOpcodeId,
    parseGlobalWsEventName,
  }
})
