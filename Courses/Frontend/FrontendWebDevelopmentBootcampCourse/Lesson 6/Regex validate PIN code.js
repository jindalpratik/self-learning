function validatePIN(pin) {
    if(!(pin.length == 4 || pin.length == 6)) {return false}
    return (!/[^0-9]+/.test(pin))
}