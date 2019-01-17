use crate::tagging::Objects;
use crate::tagging::Object;
use crate::sqlite_tagging::object::SqliteObj;

pub struct SqliteObjs {
    pub connection: sqlite::Connection
}

impl Objects for SqliteObjs {
    fn add_object(&self, uri: &str) -> &Object {
        let statement = self.connection.prepare(
            "INSERT INTO objects(uri) VALUES (?);"
        );
        return &SqliteObj {
            uri: "https://silverhetch.com"
        };
    }
}