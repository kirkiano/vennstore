use std::path::PathBuf;

use util::{Filename, ProperPathBuf};
use vs::{Result, Tag, Hash, Set, Store, Tree};
use crate::Command;



pub fn exec<S: Store>(s: &S, c: Command) -> Result<()> {
    match c {

        Command::Insert { file } =>
            exec_insert(s, file),

        Command::Tag { file, tag } =>
            exec_tag(s, file, tag),

        Command::Tags { file } =>
            exec_tags(s, file),

        Command::Find { tag } =>
            exec_find(s, tag),

        Command::Remove { file, dest } =>
            exec_remove(s, file, dest),
    }
}


fn exec_insert<S: Store>(s: &S, file: PathBuf) -> Result<()> {
    let pp = ProperPathBuf::try_from(file).unwrap();
    let f: Filename = s.insert(&pp)?;
    let ftp: Tree = s.get();
    println!("Inserted: {}/{}", ftp.as_ref(), Hash::filename_path(&f));
    Ok(())
}


fn exec_tag<S: Store>(s: &S, file: PathBuf, tag: String) -> Result<()> {
    let f = Filename::try_from(&file).unwrap();
    let t = Tag::from(tag);
    let ft = (f, t);
    s.insert(&ft)?;
    println!("Tagged {} with {}", ft.0, ft.1);
    Ok(())
}


fn exec_tags<S: Store>(s: &S, file: PathBuf) -> Result<()> {
    let f = Filename::try_from(&file).unwrap();
    let ts: Set<Tag> = s.lookup(&f)?;
    let mut vts = Vec::from_iter(ts.into_iter()).into_boxed_slice();
    vts.sort();
    for t in vts { println!("{}", t); }
    Ok(())
}


fn exec_find<S: Store>(s: &S, tag: String) -> Result<()> {
    let t = Tag::from(tag);
    let fs: Set<Filename> = s.lookup(&t)?;
    let mut vfs = Vec::from_iter(fs.into_iter()).into_boxed_slice();
    vfs.sort();
    for f in vfs { println!("{}", f); }
    Ok(())
}


fn exec_remove<S: Store>(s: &S, file: PathBuf, dest: PathBuf) -> Result<()> {
    let f = Filename::try_from(&file).unwrap();
    let pp = ProperPathBuf::try_from(dest).unwrap();
    s.remove(&f, &pp)?;
    println!("{} is now at {}", file.display(), pp);
    Ok(())
}
