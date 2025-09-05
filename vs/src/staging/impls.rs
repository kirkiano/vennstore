use std::{ffi::OsString,
          io, fs,
          path::{Path, PathBuf}};

use util::{PathOf, Filename};
use crate::traits::Lookup;
use super::Staging;



impl PathOf<Filename> for Staging {
    fn path_of(&self, f: &Filename) -> PathBuf {
        let p: &Path = self.as_ref();
        let mut pb = PathBuf::from(p);
        let fos: &OsString = f.as_ref();
        pb.push(fos);
        pb
    }
}


impl Lookup<(), Vec<PathBuf>, io::Error> for Staging {

    fn lookup(&self, _: &()) -> io::Result<Vec<PathBuf>> {
        let p: &Path = self.as_ref();
        let mut result = vec![];
        if fs::exists(p)? {
            for entry in fs::read_dir(p)? {
                let path = entry?.path();
                if path.is_file() {
                    result.push(path)
                }
            }
        }
        Ok(result)
    }
}