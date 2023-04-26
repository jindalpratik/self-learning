function removeChar(str) {
  let arraystr = str.split("");
  let newstr = "";
  for (let i = 1; i < arraystr.length - 1; i++) {
    newstr += arraystr[i];
  }
  return newstr;
}
