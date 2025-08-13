
#[cfg(test)]
pub use tests::TestDelete;



pub trait Delete<T, E> {
    fn delete(&self, x: &T) -> Result<(), E>;
}



#[cfg(test)]
mod tests {

    use std::fmt;

    use crate::{traits::{Insert, Delete, Exists},
                test_utils::Mock};


    impl<X, T, R, E> TestDelete<T, R, E> for X
    where X: Insert<T, R, E> + Delete<R, E> + Exists<R, E>,
          T: Mock,
          E: fmt::Debug + Eq,
    {}


    pub trait TestDelete<T, R, E>:
        Insert<T, R, E> + Delete<R, E> + Exists<R, E>
    where T: Mock,
          E: fmt::Debug + Eq,
    {
        fn test_delete(&self) -> Result<(), E> {
            let x = T::mock(());

            let r = self.insert(&x)?;
            assert!(self.exists(&r)?);

            self.delete(&r)?;
            assert!(!self.exists(&r)?);

            let double_deletion_result = self.delete(&r);
            assert!(double_deletion_result.is_err());
            let _ = double_deletion_result
                .map_err(|e| assert!(self.is_does_not_exist_error(&e)));
            Ok(())
        }
    }
}
