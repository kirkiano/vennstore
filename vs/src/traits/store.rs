use std::fmt;

use util::{Has, Filename, ProperPathBuf};
use crate::{Error, Set, Tag, Tree,
            traits::{Insert, Lookup, Remove}};



pub trait Store:
    fmt::Display
  + Has<Tree>
  + Insert<ProperPathBuf, Filename, Error>
  + Insert<(Filename, Tag), (), Error>
  + Lookup<Tag, Set<Filename>, Error>
  + Lookup<Filename, Set<Tag>, Error>
  + Remove<Filename, ProperPathBuf, Error>
{}


impl<X> Store for X
where X: fmt::Display
       + Has<Tree>
       + Insert<ProperPathBuf, Filename, Error>
       + Insert<(Filename, Tag), (), Error>
       + Lookup<Tag, Set<Filename>, Error>
       + Lookup<Filename, Set<Tag>, Error>
       + Remove<Filename, ProperPathBuf, Error>
{}
