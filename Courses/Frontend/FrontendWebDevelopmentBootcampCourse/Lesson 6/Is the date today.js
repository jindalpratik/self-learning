function isToday(date) {
  let currdate = new Date();
  return (
    date.getDate() == currdate.getDate() && date.getDay() == currdate.getDay()
  );
}
