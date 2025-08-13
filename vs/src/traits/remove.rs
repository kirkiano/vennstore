
#[cfg(test)]
pub use tests::TestRemove;



pub trait Remove<T, D, E> {
    fn remove(&self, x: &T, dest: &D) -> Result<(), E>;
}



#[cfg(test)]
mod tests {

    use std::fmt;

    use crate::{traits::{Insert, Remove, Exists},
                test_utils::Mock};


    impl<X, T, R, E> TestRemove<T, R, E> for X
    where X: Insert<T, R, E> + Remove<R, T, E> + Exists<R, E>,
          T: Mock,
          E: fmt::Debug + Eq,
    {}


    pub trait TestRemove<T, R, E>:
        Insert<T, R, E> + Remove<R, T, E> + Exists<R, E>
    where T: Mock,
          E: fmt::Debug + Eq,
    {
        fn test_remove(&self) -> Result<(), E> {
            let x = T::mock(());

            let r = self.insert(&x)?;
            assert!(self.exists(&r)?);

            self.remove(&r, &x)?;
            assert!(!self.exists(&r)?);

            let double_removal_result = self.remove(&r, &x);
            assert!(double_removal_result.is_err());
            let _ = double_removal_result
                .map_err(|e| assert!(self.is_does_not_exist_error(&e)));
            Ok(())
        }
    }
}
