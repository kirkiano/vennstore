use std::{fs,
          io,
          path::Path};

use util::{Filename, ProperPathBuf};
use crate::traits::{Insert, Exists, AssertExists, Remove};
use super::{Tree, Result, Error};



impl Exists<Filename, Error> for Tree {

    fn is_does_not_exist_error(&self, e: &Error) -> bool {
        matches!(e, Error::NotFound(_))
    }

    fn is_already_exists_error(&self, e: &Error) -> bool {
        matches!(e, Error::AlreadyExists(_))
    }

    fn exists(&self, f: &Filename) -> Result<bool> {
        let p = self.filename_path(f);
        fs::exists(p)
           .map_err(|e| {
               let not_found = e.kind() == io::ErrorKind::NotFound;
               if not_found { Error::NotFound(f.clone()) }
               else { Error::internal(e) }
           })
    }
}


impl AssertExists<Filename, Error> for Tree {
    fn assert_exists(&self, f: &Filename) -> Result<()> {
        if !self.exists(f)? { Err(Error::NotFound(f.clone())) }
        else { Ok(()) }
    }

    fn assert_does_not_exist(&self, f: &Filename) -> Result<()> {
        if self.exists(f)? { Err(Error::AlreadyExists(f.clone())) }
        else { Ok(()) }
    }
}


impl Insert<ProperPathBuf, Filename, Error> for Tree {
    fn insert(&self, from: &ProperPathBuf) -> Result<Filename> {
        let f: &Filename = from.as_ref();
        let to = self.filename_path(&f);
        fs::create_dir_all(to.parent()).map_err(Error::internal)?;
        self.assert_does_not_exist(f)?;
        fs::rename(&from, &to).map_err(Error::cannot_move(&from, &to))?;
        Ok(f.clone())
    }
}


impl Remove<Filename, ProperPathBuf, Error> for Tree {
    fn remove(&self, f: &Filename, dest: &ProperPathBuf) -> Result<()> {
        self.assert_exists(f)?;
        let pp = self.filename_path(f);
        let src: &Path = pp.as_ref();

        let dst_dir: &Path = dest.as_ref();
        if !dst_dir.is_dir() { Err(Error::not_a_dir(dest))? }

        let dst_file = dest.clone() + f;
        fs::rename(src, dst_file).map_err(Error::internal)
    }
}
