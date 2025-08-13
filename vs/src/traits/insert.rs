
#[cfg(test)]
pub use tests::TestInsert;



pub trait Insert<T, R, E> {
    fn insert(&self, x: &T) -> Result<R, E>;
}



#[cfg(test)]
mod tests {

    use std::fmt;

    use crate::{traits::{Insert, Exists},
                test_utils::Mock};


    impl<X, T, R, E> TestInsert<T, R, E> for X
    where X: Insert<T, R, E> + Exists<R, E>,
          T: Mock,
          R: Clone,
          E: fmt::Debug + Eq,
    {}


    pub trait TestInsert<T, R, E>:
        Insert<T, R, E> + Exists<R, E>
    where T: Mock,
          R: Clone,
          E: fmt::Debug + Eq,
    {
        fn test_insert(&self) -> Result<(), E> {
            let x = T::mock(());

            let r = self.insert(&x)?;
            assert!(self.exists(&r)?);

            let double_insertion_result = self.insert(&x);
            assert!(double_insertion_result.is_err());
            let _ = double_insertion_result
                .map_err(|e| assert!(self.is_already_exists_error(&e)));
            Ok(())
        }
    }
}
