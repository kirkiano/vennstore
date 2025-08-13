use std::{fs::File,
          path::Path};

use dirs::home_dir;

use util::{time::now_nanos, Filename, ProperPathBuf};



pub fn test_root_path() -> ProperPathBuf {
    let mut p = home_dir().unwrap();
    p.push("tmp");
    p.push("vennstore_test");
    p.try_into().unwrap()
}


/////////////////////////////////////////////////////////////////////
/// mock

pub trait Mock<Params=(), Result=Self> {
    fn mock(params: Params) -> Result;
}


impl Mock<()> for Filename {
    fn mock(_: ()) -> Self {
        let s = format!("dummy_filename_{}", now_nanos());
        Self::from(s.as_str())
    }
}

impl Mock<(), ProperPathBuf> for Filename {
    fn mock(_: ()) -> ProperPathBuf {
        let f: Self = Self::mock(());
        f.into()
    }
}

impl Mock<()> for ProperPathBuf {
    fn mock(_: ()) -> Self {
        Self::mock(true)
    }
}

impl Mock<bool> for ProperPathBuf {
    fn mock(do_create: bool) -> ProperPathBuf {
        let fname: ProperPathBuf = Filename::mock(());
        let pp = test_root_path() + &fname;
        let p: &Path = pp.as_ref();
        if do_create { File::create(p).unwrap(); } // creates an empty file
        pp
    }
}
