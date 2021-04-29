mod array;
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
}
