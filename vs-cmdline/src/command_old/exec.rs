
use crate::{Command, Result, Response,
            api_eventually_to_be_deleted::{init, add_file, tag_file}};


impl Command {
    pub fn exec(&self) -> Result<Response> {
        match self {

            Command::Init =>
                { init(); Ok(Response::Null) },

            Command::AddFile(p) =>
                add_file(p).map(Response::FileAdded),

            Command::TagFile(fl, t) =>
                tag_file(fl, t)
                    .map(|()| Response::FileTagged(fl.clone(), t.clone())),
        }
    }
}
