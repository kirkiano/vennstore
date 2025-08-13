
use crate::Hash;
use util::{Filename, ProperPathBuf};


/// The root of the file tree
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree(ProperPathBuf);



impl From<ProperPathBuf> for Tree {
    fn from(p: ProperPathBuf) -> Self {
        Self(p)
    }
}


/// The root path
impl AsRef<ProperPathBuf> for Tree {
    fn as_ref(&self) -> &ProperPathBuf {
        &self.0
    }
}


impl Tree {

    pub fn filename_path(&self, f: &Filename) -> ProperPathBuf {
        let h = Hash::from(f);
        self.hash_path(&h) + f
    }

    fn hash_path(&self, h: &Hash) -> ProperPathBuf {
        self.as_ref().clone() + &h.path()
    }
}
