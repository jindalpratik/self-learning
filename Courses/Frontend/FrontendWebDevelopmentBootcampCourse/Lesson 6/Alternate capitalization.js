function capitalize(s) {
  return [
    s
      .split("")
      .map((element, index) => {
        if (index % 2 == 0) {
          return element.toUpperCase();
        } else {
          return element;
        }
      })
      .join(""),
    s
      .split("")
      .map((element, index) => {
        if (index % 2 == 1) {
          return element.toUpperCase();
        } else {
          return element;
        }
      })
      .join(""),
  ];
}
