use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct Directory {
  path: PathBuf,
}

impl Directory {
  pub fn new(path: PathBuf) -> Self {
    Directory { path }
  }

  pub fn join(&self, name: &str) -> Directory {
    Directory::new(self.path.join(name))
  }

  pub fn path(&self) -> &PathBuf {
    &self.path
  }

  pub fn reset(&self) -> Result<(), std::io::Error> {
    drop(fs::remove_dir_all(&self.path));
    fs::create_dir_all(&self.path)
  }

  pub fn write_file(&self, name: &str, content: &str) -> Result<(), std::io::Error> {
    let target = self.path.join(name);
    let parent = target.parent().unwrap();
    fs::create_dir_all(parent)?;
    fs::write(target, content)
  }
}
