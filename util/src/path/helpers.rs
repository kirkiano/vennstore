use std::{fs::{self, DirBuilder},
          path::Path};



pub fn assert_path_existence<P: AsRef<Path>>(path: P, exists: bool) {
    let exists_result = fs::exists(path.as_ref());
    assert!(exists_result.is_ok());
    let _ = exists_result.map(|b| assert_eq!(b, exists));
}


pub fn ensure_dir_created<P: AsRef<Path>>(p: P) {
    let ps = p.as_ref().to_string_lossy();

    DirBuilder::new()
        .recursive(true)
        .create(p.as_ref())
        .expect(&format!("Cannot create directory {}", ps))
}
