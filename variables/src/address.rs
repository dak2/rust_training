pub fn test() {
  let hoge: i8 = 100;
  let reference: &i8 = &hoge;
  println!("{:p}", reference);
}
