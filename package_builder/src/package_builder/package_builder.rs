use std::process::Command;

use crate::{Directory, Package, PackageGenerator, Workspace};

#[derive(Debug)]
pub struct PackageBuilder {
  error: Option<String>,
  generated: bool,
  output: Option<String>,
  package: Package,
  root: Directory,
}

impl PackageBuilder {
  pub fn try_new(folder: &str, package: Package) -> Result<Self, std::io::Error> {
    let root = Workspace::target_dir().join(folder).join(package.name());
    root.reset()?;
    Ok(PackageBuilder { error: None, generated: false, output: None, root, package })
  }

  pub fn build(&mut self) -> Result<(), std::io::Error> {
    self.generate()?;

    let output = Command::new("cargo").current_dir(&self.root.path()).arg("build").output()?;

    self.error = Some(std::str::from_utf8(&output.stderr).unwrap().to_string());
    self.output = Some(std::str::from_utf8(&output.stdout).unwrap().to_string());

    Ok(())
  }

  pub fn expand(&mut self) -> Result<(), std::io::Error> {
    self.generate()?;

    let output = Command::new("cargo").current_dir(&self.root.path()).arg("expand").output()?;

    self.error = Some(std::str::from_utf8(&output.stderr).unwrap().to_string());
    self.output = Some(std::str::from_utf8(&output.stdout).unwrap().to_string());

    Ok(())
  }

  pub fn generate(&mut self) -> Result<(), std::io::Error> {
    if !self.generated {
      PackageGenerator::generate(&self.root, &self.package)?;
      self.generated = true;
    }
    Ok(())
  }

  pub fn error(&self) -> &Option<String> {
    &self.error
  }

  pub fn output(&self) -> &Option<String> {
    &self.output
  }
}
