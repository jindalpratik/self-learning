function repeatStr(n, s) {
  let newstr = "";
  while (n > 0) {
    newstr += s;
    n--;
  }
  return newstr;
}
