use std::fs;

use util::{Filename, ProperPathBuf};
use crate::{test_utils::{Mock, test_root_path},
            traits::{Insert, Remove, Exists, AssertExists, TestInsert},
            FILES_DIR_NAME};
use super::{Result, Error, Tree};



#[test]
fn test_file_tree_insert() -> Result<()> {
    let r = prepare_test_file_tree_path()?;
    r.test_insert()?;
    Ok(())
}


#[test]
fn test_file_tree_remove() -> Result<()> {
    let r = prepare_test_file_tree_path()?;

    let pp = ProperPathBuf::mock(());

    let par = pp.parent();
    let parent = ProperPathBuf::try_from(par.to_owned()).unwrap();

    let f = r.insert(&pp)?;
    r.assert_exists(&f)?;

    r.remove(&f, &parent)?;
    r.assert_does_not_exist(&f)?;

    let double_removal_result = r.remove(&f, &parent);
    assert!(double_removal_result.is_err());
    let _ = double_removal_result
        .map_err(|e| assert!(r.is_does_not_exist_error(&e)));
    Ok(())
}


fn prepare_test_file_tree_path() -> Result<Tree> {
    let p = test_file_tree_path();
    fs::create_dir_all(&p).map_err(Error::internal)?;
    Ok(p.into())
}


fn test_file_tree_path() -> ProperPathBuf {
    test_root_path() + &Filename::from(FILES_DIR_NAME)
}
