function persistence(num) {
  let mul = num;
  let rem = 0;
  let temp_mul = 1;
  let count = 0;
  while (mul > 9) {
    for (let i = mul; i != 0; i = (i - rem) / 10) {
      rem = i % 10;
      temp_mul = temp_mul * rem;
    }
    mul = temp_mul;
    count += 1;
    temp_mul = 1;
    rem = 0;
  }
  return count;
}
