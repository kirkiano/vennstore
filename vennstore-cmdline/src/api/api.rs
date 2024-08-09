use std::{fs::rename,
          path::{Path, PathBuf},
          os::unix::fs::symlink};

use crate::{Error, Result,
            Tag, Filename,
            FILE_DIR, TAGS_DIR,
            ensure_dir_created};


/// Initialize a VennStore at the current dir
pub fn init() {
    for path in [FILE_DIR, TAGS_DIR] {
        ensure_dir_created(path)
    }
}


pub fn add_file(src_path: &String) -> Result<Filename> {
    let sp = PathBuf::from(src_path.clone());
    let fname = sp.file_name().unwrap().to_string_lossy().to_string();
    let filename = Filename::from(fname);

    // create hashed path dir
    let hp = filename.hashed_path();
    let pp = hp.parent().unwrap();
    ensure_dir_created(pp);

    // move the file into the hashed path dir
    let target_path = pp.join(filename.to_string());
    if target_path.exists() {
        return Err(Error::FileExists(filename));
    }
    rename(src_path.clone(), target_path)
        .map_err(Error::cannot_install_file(src_path.clone()))?;

    Ok(filename)
}


pub fn tag_file(f: &Filename, t: &Tag) -> Result<()> {
    let hp = f.hashed_path();
    if !hp.exists() { return Err(Error::NoSuchFile(f.clone())); }

    let original = Path::new("..").join("..").join(hp);
    t.ensure_created();
    let link = t.path().join(f.to_string());
    symlink(original.clone(), link.clone())
        .map_err(Error::cannot_symlink(original, link))
}
