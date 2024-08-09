use std::{fs::DirBuilder,
          path::Path};


pub fn ensure_dir_created<P: AsRef<Path>>(p: P) {
    let ps = p.as_ref().to_string_lossy();

    DirBuilder::new()
        .recursive(true)
        .create(p.as_ref())
        .expect(&format!("Cannot create directory {}", ps))
}
