use std::{ffi::OsString, io, fs, path::PathBuf};

use util::Filename;



#[derive(Debug)]
pub struct VSSqliteTree {
    root: PathBuf,
}


impl VSSqliteTree {

    fn staging_dir_path(&self) -> PathBuf {
        let mut p = self.root.clone();
        p.push(".staging");
        p
    }

    fn staging_path(&self, f: &Filename) -> PathBuf {
        let mut p = self.staging_dir_path();
        let fos: &OsString = f.as_ref();
        p.push(fos);
        p
    }

    fn find_files_in_staging(&self) -> io::Result<Vec<Filename>> {
        let mut result = vec![];
        let staging_path = self.staging_dir_path();
        if fs::exists(&staging_path)? {
            for entry in fs::read_dir(staging_path)? {
                let path = entry?.path();
                if path.is_dir() {
                    result.push(Filename::try_from(&path).unwrap())
                }
            }
        }
        Ok(result)
    }
}
