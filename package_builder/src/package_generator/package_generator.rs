use crate::{Directory, Package};

pub struct PackageGenerator {}

impl PackageGenerator {
  pub fn generate(root: &Directory, package: &Package) -> Result<(), std::io::Error> {
    Self::generate_cargo(root, package)?;
    root.write_file("src/lib.rs", "pub fn main() {}")
  }

  pub fn generate_cargo(root: &Directory, package: &Package) -> Result<(), std::io::Error> {
    let content = format!(
      "[package]
name = '{}'
version = '0.1.0'
edition = '2021'

[workspace]
",
      package.name()
    );

    root.write_file("Cargo.toml", &content)
  }
}
