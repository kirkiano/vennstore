
use rusqlite::Connection;

use util::{Filename, ProperPathBuf};
use crate::{Tag, Set, FT,
            sqlite::{self, Error},
            test_utils::{Mock, test_root_path},
            traits::{Insert, TestInsert,
                     Lookup,
                     Delete, TestDelete},
            SQLITE_DB_NAME};
use super::Result;



type C = Connection;


#[test]
fn test_sqlite() -> Result<()> {
    let db = sqlite::create_idempotently(&sqlite_test_db_path())?;

    // insertions
    <C as TestInsert<Tag, Tag, Error>>::test_insert(&db)?;
    <C as TestInsert<Filename, Filename, Error>>::test_insert(&db)?;

    // deletions
    <C as TestDelete<Tag, Tag, Error>>::test_delete(&db)?;
    <C as TestDelete<Filename, Filename, Error>>::test_delete(&db)?;

    Ok(())
}


#[test]
fn test_sqlite_file_tree_tagging() -> Result<()> {
    let db = sqlite::create_idempotently(&sqlite_test_db_path())?;

    let fta = insert_mock_file_and_tag(&db)?;
    let ftb = insert_mock_file_and_tag(&db)?;

    let (fsa, tsa) = sets(&db, &fta)?;
    let (fsb, tsb) = sets(&db, &ftb)?;

    assert_eq!(1, fsa.len());
    assert_eq!(1, tsa.len());

    assert_eq!(1, fsb.len());
    assert_eq!(1, tsb.len());

    <C as Delete<FT, Error>>::delete(&db, &fta)?;
    <C as Delete<FT, Error>>::delete(&db, &ftb)?;
    <C as Delete<Tag, Error>>::delete(&db, &fta.1)?;
    <C as Delete<Tag, Error>>::delete(&db, &ftb.1)?;
    <C as Delete<Filename, Error>>::delete(&db, &fta.0)?;
    <C as Delete<Filename, Error>>::delete(&db, &ftb.0)?;

    Ok(())
}


fn insert_mock_file_and_tag(db: &C) -> Result<FT> {
    let f = Filename::mock(());
    let t = Tag::mock(());

    <C as Insert<Filename, Filename, Error>>::insert(&db, &f)?;
    <C as Insert<Tag, Tag, Error>>::insert(&db, &t)?;

    let ft = (f, t);
    <C as Insert<FT, (), Error>>::insert(&db, &ft)?;

    Ok(ft)
}


fn sets(db: &C, ft: &FT) -> Result<(Set<Filename>, Set<Tag>)> {
    let p = (<C as Lookup<Tag, Set<Filename>, Error>>::lookup(&db, &ft.1)?,
             <C as Lookup<Filename, Set<Tag>, Error>>::lookup(&db, &ft.0)?);
    Ok(p)
}


fn sqlite_test_db_path() -> ProperPathBuf {
    test_root_path() + &Filename::from(SQLITE_DB_NAME)
}
