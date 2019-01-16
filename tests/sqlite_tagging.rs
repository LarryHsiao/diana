extern crate diana;

use diana::sqlite_tagging::*;
use diana::tagging::Objects;

#[test]
fn object_adding() {
    let objs = SqliteObjs {
        connection: sqlite::open(":memory:").unwrap()
    };
    let result = objs.add_object("https://silverhetch.com");

    assert_eq!("https://silverhetch.com",result.uri())
}