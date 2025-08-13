

pub trait Lookup<T, R, E> {
    fn lookup(&self, x: &T) -> Result<R, E>;
}
