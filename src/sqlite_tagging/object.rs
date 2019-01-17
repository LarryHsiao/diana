use crate::tagging::Object;


pub struct SqliteObj<'a> {
    pub uri: &'a str
}

impl<'a> Object for SqliteObj<'a> {
    fn uri(&self) -> &str {
        return self.uri;
    }
}