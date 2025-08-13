use std::fs;

use util::{Filename, ProperPathBuf};
use crate::{Set, Tag, FT, SFT,
            traits::{Insert, TestInsert, Remove, Delete, TestDelete, Lookup},
            test_utils::{Mock, test_root_path}};
use super::{Result, Error};



#[test]
fn test_sqlite_file_tree() -> Result<()> {
    let p = test_root_path();
    fs::create_dir_all(&p).map_err(Error::internal)?;
    let sft = SFT::create_idempotently(p)?;

    // insertions
    <SFT as TestInsert<Tag, Tag, Error>>::test_insert(&sft)?;
    <SFT as TestInsert<ProperPathBuf, Filename, Error>>::test_insert(&sft)?;

    // removals
    <SFT as TestDelete<Tag, Tag, Error>>::test_delete(&sft)?;
    // <SFT as TestRemove<ProperPathBuf, Filename, Error>>::test_remove(&sft)?;

    Ok(())
}


#[test]
fn test_file_tree_remove() -> Result<()> {
    let r = test_root_path();
    fs::create_dir_all(&r).map_err(Error::internal)?;
    let sft = SFT::create_idempotently(r)?;

    let pp = ProperPathBuf::mock(());

    let par = pp.parent();
    let parent = ProperPathBuf::try_from(par.to_owned()).unwrap();

    let f = sft.insert(&pp)?;

    sft.remove(&f, &parent)?;

    Ok(())
}


#[test]
fn test_sqlite_file_tree_tagging() -> Result<()> {
    let p = ProperPathBuf::mock(());
    let t = Tag::mock(());

    let r = test_root_path();
    fs::create_dir_all(&r).map_err(Error::internal)?;
    let sft = SFT::create_idempotently(r)?;

    let f = <SFT as Insert<ProperPathBuf, Filename, Error>>::insert(&sft, &p)?;
    <SFT as Insert<Tag, Tag, Error>>::insert(&sft, &t)?;

    let ft = (f, t);

    let fs0 = <SFT as Lookup<Tag, Set<Filename>, Error>>::lookup(&sft, &ft.1)?;
    let ts0 = <SFT as Lookup<Filename, Set<Tag>, Error>>::lookup(&sft, &ft.0)?;

    <SFT as Insert<FT, (), Error>>::insert(&sft, &ft)?;

    let fs1 = <SFT as Lookup<Tag, Set<Filename>, Error>>::lookup(&sft, &ft.1)?;
    let ts1 = <SFT as Lookup<Filename, Set<Tag>, Error>>::lookup(&sft, &ft.0)?;

    assert_eq!(Set::from_iter(vec![ft.1.clone()].iter()),
               ts1.difference(&ts0).collect());

    assert_eq!(Set::from_iter(vec![ft.0.clone()].iter()),
               fs1.difference(&fs0).collect());

    let f = ft.0;
    let t = ft.1;

    let ppar = ProperPathBuf::try_from(p.parent().to_owned()).unwrap();
    <SFT as Delete<Tag, Error>>::delete(&sft, &t)?;
    <SFT as Remove<Filename, ProperPathBuf, Error>>::remove(&sft, &f, &ppar)?;

    Ok(())
}
