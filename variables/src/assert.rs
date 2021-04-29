use proconio::input;

pub fn test() {
  input! {
    x: i32,
  }
  let r = x % 10;
  assert!(0 <= r && r < 10);
  println!("あまりは {}", r);
}
