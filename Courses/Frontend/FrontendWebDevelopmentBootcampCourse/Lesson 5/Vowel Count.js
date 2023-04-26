function getCount(str) {
  let arraystr = str.split("");
  let count = 0;
  for (let i = 0; i < arraystr.length; i++) {
    if (
      arraystr[i] === "a" ||
      arraystr[i] === "e" ||
      arraystr[i] === "i" ||
      arraystr[i] === "o" ||
      arraystr[i] === "u"
    ) {
      count++;
    }
  }
  return count;
}
