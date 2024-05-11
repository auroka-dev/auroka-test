use crate::Directory;
use std::env;

pub struct Workspace {}

impl Workspace {
  pub fn target_dir() -> Directory {
    let mut target = env::current_exe().unwrap();
    target.pop();
    if target.ends_with("deps") {
      target.pop();
      target.pop(); // profile - debug or release
    }
    Directory::new(target)
  }
}
