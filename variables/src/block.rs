pub fn test() {
  println!("ブロックの前");
  let hoge = {
      println!("ブロックの中");
      10
  };
  println!("ブロックの後， hoge の値は {}", hoge);
}
