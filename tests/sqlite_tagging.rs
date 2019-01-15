extern crate Diana;

use Diana::sqlite_tagging::*;
use Diana::tagging::Objects;

#[test]
fn object_adding() {
    let objs = SqliteObjs {
        connection: sqlite::open(":memory:").unwrap()
    };
    let result = objs.add_object("https://silverhetch.com");

    assert_eq!("https://silverhetch.com",result.uri())
}