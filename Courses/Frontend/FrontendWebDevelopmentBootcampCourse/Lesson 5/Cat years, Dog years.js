var humanYearsCatYearsDogYears = function (humanYears) {
  let catYears = 15;
  let dogYears = 15;
  if (humanYears >= 2) {
    catYears += 9;
    dogYears += 9;
  }
  if (humanYears >= 3) {
    let temphumanyears = humanYears;
    while (temphumanyears >= 3) {
      catYears += 4;
      dogYears += 5;
      temphumanyears--;
    }
  }

  return [humanYears, catYears, dogYears];
};
