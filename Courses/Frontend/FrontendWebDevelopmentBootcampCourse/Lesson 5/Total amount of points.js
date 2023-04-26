function points(games) {
  let points_scored = 0;
  let temp_array = [];
  for (let i = 0; i < games.length; i++) {
    temp_array = games[i].split(":");
    if (temp_array[0] > temp_array[1]) {
      points_scored += 3;
    } else if (temp_array[0] == temp_array[1]) {
      points_scored += 1;
    }
  }
  return points_scored;
}
