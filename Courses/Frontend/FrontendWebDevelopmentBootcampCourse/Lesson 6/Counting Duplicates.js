function duplicateCount(text) {
  const counts = {};
  const newtext = text.toLowerCase();
  const textarr = newtext.split("");
  textarr.forEach((x) => {
    counts[x] = (counts[x] || 0) + 1;
  });
  // console.log(counts)
  let newcount = 0;
  for (let newele in counts) {
    if (counts[newele] > 1) {
      newcount += 1;
    }
  }
  return newcount;
}
