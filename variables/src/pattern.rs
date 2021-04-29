pub fn test() {
  let tuple = (1, 2.4);
  let (x, _y) = tuple;
  assert_eq!(x, 1)
}
