function arrayDiff(a, b) {
  lenb = b.length;
  let newArr = a;
  for (let i = 0; i < lenb; i++) {
    newArr = newArr.filter((arrElement) => arrElement != b[i]);
  }
  return newArr;
}
