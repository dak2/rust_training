mod address;
mod array;
mod array_address;
mod assert;
mod block;
mod condition;
mod forloop;
mod pattern;
mod ref_pattern;
mod sum;
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
  forloop::test();
  ref_pattern::test();
  sum::test();
}
