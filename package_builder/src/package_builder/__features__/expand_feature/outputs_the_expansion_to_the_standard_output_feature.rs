use crate::package_builder::__steps__::given_there_is_a_package_builder;
use crate::package_builder::__steps__::then_the_standard_output_should_have;
use crate::package_builder::__steps__::when_expand_is_invoked;
use crate::package_builder::__steps__::Context;

#[test]
fn outputs_the_expansion_to_the_standard_output_feature() {
  let mut context = Context::new();
  given_there_is_a_package_builder(&mut context);
  when_expand_is_invoked(&mut context);
  then_the_standard_output_should_have(
    &context,
    "#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub fn main() {}",
  );
}
