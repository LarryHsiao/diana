use crate::source::Source;

pub struct SqliteConn<'a> {
    pub connection: &'a sqlite::Connection
}

impl<'a> Source<sqlite::Connection> for SqliteConn<'a> {
    fn fetch(&self) -> &sqlite::Connection {
        self.connection.execute(
            "CREATE TABLE objects(uri TEXT UNIQUE);"
        );
        return self.connection;
    }
}