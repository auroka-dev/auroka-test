#[derive(Debug)]
pub struct Package {
  name: String,
}

impl Package {
  pub fn new(name: &str) -> Self {
    Package { name: name.to_string() }
  }

  pub fn name(&self) -> &str {
    &self.name
  }
}
