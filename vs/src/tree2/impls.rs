use std::{io,
          path::{Path, PathBuf}};
use walkdir::{WalkDir, DirEntry};

use util::{PathOf, Filename};
use crate::{Hash, traits::Lookup};
use super::Tree2;



impl PathOf<Filename> for Tree2 {
    fn path_of(&self, f: &Filename) -> PathBuf {
        let fp = Hash::filename_path(f);
        let p: &Path = self.as_ref();
        let mut pb = PathBuf::from(p);
        pb.push(fp);
        pb
    }
}


impl Lookup<(), Vec<PathBuf>, io::Error> for Tree2 {

    fn lookup(&self, _: &()) -> io::Result<Vec<PathBuf>> {
        let mut result = vec![];
        for entry in WalkDir::new(self.as_ref())
            .into_iter()
            .filter_entry(|e: &DirEntry| e.file_type().is_file())
        {
            result.push(entry?.into_path())
        }
        Ok(result)
    }
}