
pub trait Exists<T, E> {

    fn is_already_exists_error(&self, e: &E) -> bool;
    fn is_does_not_exist_error(&self, e: &E) -> bool;

    fn exists(&self, x: &T) -> Result<bool, E>;
}


pub trait AssertExists<T, E> {
    fn assert_exists(&self, x: &T) -> Result<(), E>;
    fn assert_does_not_exist(&self, x: &T) -> Result<(), E>;
}
