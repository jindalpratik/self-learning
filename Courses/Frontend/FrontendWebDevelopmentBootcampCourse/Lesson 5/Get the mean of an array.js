function getAverage(marks) {
  let sum = 0;
  let arraylen = marks.length;
  for (let i = 0; i < arraylen; i++) {
    sum += marks[i];
  }
  return Math.floor(sum / arraylen);
}
