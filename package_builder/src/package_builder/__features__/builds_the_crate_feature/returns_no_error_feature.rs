use crate::package_builder::__steps__::given_there_is_a_package_builder;
use crate::package_builder::__steps__::then_the_result_should_be_ok;
use crate::package_builder::__steps__::when_build_is_invoked;
use crate::package_builder::__steps__::Context;

#[test]
fn outputs_its_running_1_test_feature() {
  let mut context = Context::new();
  given_there_is_a_package_builder(&mut context);
  when_build_is_invoked(&mut context);
  then_the_result_should_be_ok(&mut context);
}
