use std::path::PathBuf;


pub trait PathOf<T> {
    fn path_of(&self, x: &T) -> PathBuf;
}