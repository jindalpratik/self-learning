function isValidWalk(walk) {
  if (walk.length != 10) {
    return false;
  }
  let dirArray = [0, 0];
  walk.forEach((element) => {
    if (element == "n") {
      dirArray[0] = dirArray[0] + 1;
    } else if (element == "s") {
      dirArray[0] = dirArray[0] - 1;
    } else if (element == "e") {
      dirArray[1] = dirArray[1] + 1;
    } else if (element == "w") {
      dirArray[1] = dirArray[1] - 1;
    }
  });
  return dirArray[0] == 0 && dirArray[1] == 0;
}
