use rand::distributions::Alphanumeric;
use rand::Rng;
use std::assert_matches::assert_matches;

use super::super::Context;
use crate::{Package, PackageBuilder};

pub fn given_there_is_a_package_builder(context: &mut Context) {
  let rng = rand::thread_rng();
  let suffix: String = rng.sample_iter(&Alphanumeric).take(5).map(char::from).collect();

  let package = Package::new("test");

  let package_builder = PackageBuilder::try_new(&format!("package_builder_tests_{}", suffix), package);

  assert_matches!(package_builder, Ok(_), "Expected Ok(_) but got {:?}", package_builder);

  context.package_builder_set(package_builder.unwrap());
}
