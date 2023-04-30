function filter_list(l) {
  return l.filter(arrayElement => !(typeof arrayElement == 'string'));
}