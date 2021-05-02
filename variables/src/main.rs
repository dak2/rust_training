mod address;
mod array;
mod array_address;
mod assert;
mod block;
mod condition;
mod pattern;
mod tuple;

fn main() {
  condition::test();
  block::test();
  tuple::test();
  assert::test();
  pattern::test();
  array::test();
  address::test();
  array_address::test();
}
