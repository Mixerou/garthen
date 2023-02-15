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

  function parseGlobalApiErrorCode(code) {
    return Object.keys(GLOBAL_API_ERRORS).find(
      key => GLOBAL_API_ERRORS[key] === code
    )
  }

  return {
    GLOBAL_API_ERRORS,
    parseGlobalApiErrorCode,
  }
})
