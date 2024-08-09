
use crate::{Command, Result, Response};
use super::api::{init, add_file, tag_file};


pub fn exec(c: &Command) -> Result<Response> {
    match c {

        Command::Init =>
            { init(); Ok(Response::Null) },

        Command::AddFile(p) =>
            add_file(p).map(Response::FileAdded),

        Command::TagFile(fl, t) =>
            tag_file(fl, t)
                .map(|()| Response::FileTagged(fl.clone(), t.clone())),
    }
}
