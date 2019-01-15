use crate::tagging::Object;
use crate::tagging::Objects;

pub struct SqliteObjs {
    pub connection: sqlite::Connection
}

impl Objects for SqliteObjs {
    fn add_object(&self, uri: &str) -> Box<Object> {
        return Box::new(SqliteObj {
            uri: "https://silverhetch.com"
        });
    }
}

pub struct SqliteObj<'a> {
    uri: &'a str
}

impl<'a> Object for SqliteObj<'a> {
    fn uri(&self) -> &str {
        return &self.uri;
    }
}