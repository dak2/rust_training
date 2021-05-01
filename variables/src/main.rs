mod array;
mod assert;
mod block;
mod condition;
mod pattern;
mod tuple;
mod address;

fn main() {
  condition::test();
  block::test();
  tuple::test();
  assert::test();
  pattern::test();
  array::test();
  address::test();
}
