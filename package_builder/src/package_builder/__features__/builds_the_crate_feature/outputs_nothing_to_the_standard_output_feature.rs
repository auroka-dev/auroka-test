use crate::package_builder::__steps__::given_there_is_a_package_builder;
use crate::package_builder::__steps__::then_the_standard_output_should_be_empty;
use crate::package_builder::__steps__::when_build_is_invoked;
use crate::package_builder::__steps__::Context;

#[test]
fn outputs_nothing_to_the_standard_output_feature() {
  let mut context = Context::new();
  given_there_is_a_package_builder(&mut context);
  when_build_is_invoked(&mut context);
  then_the_standard_output_should_be_empty(&context);
}
