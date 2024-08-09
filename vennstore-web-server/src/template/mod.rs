use std::fmt;

use serde::Serialize;
use tera;
use once_cell::sync::Lazy;


///////////////////////////////////////////////////////////
// templates

pub static TEMPLATE: Lazy<Template> = Lazy::new(|| Template::new());


#[derive(Debug)]
pub struct Template {
    inn: ::tera::Tera,
}


impl Template {
    pub fn new() -> Self {
        let inn = tera::Tera::new("src/template/**/*.html")
            .expect("Can't load templates");
        Self { inn }
    }

    pub fn render(&self, template_path: &str, c: &Context) ->
        Result<String, Error>
    {
        let path = format!("pages/{}", template_path);
        self.inn.render(&path, &c.inn)
            .map_err(Error::from)
    }

    pub fn render_serializable<S>(&self, template_path: &str, s: &S) ->
        Result<String, Error>
    where S: Serialize
    {
        let c = tera::Context::from_serialize(s)
            .map_err(Error::from)?
            .into();
        self.render(template_path, &c)
    }

}


///////////////////////////////////////////////////////////
// context

#[derive(Debug)]
pub struct Context {
    inn: tera::Context,
}


impl From<tera::Context> for Context {
    fn from(inn: tera::Context) -> Self {
        Self { inn }
    }
}


impl Context {
    pub fn new() -> Self {
        Self { inn: tera::Context::new() }
    }

    pub fn bind<T>(mut self, var: impl Into<String>, val: &T) -> Self
    where T: Serialize + ?Sized
    {
        self.inn.insert(var, val);
        self
    }
}


///////////////////////////////////////////////////////////
// error

#[derive(Debug)]
pub struct Error {
    inn: tera::Error,
}

impl From<tera::Error> for Error {
    fn from(inn: tera::Error) -> Self {
        Self { inn }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inn.fmt(f)
    }
}


impl std::error::Error for Error {}