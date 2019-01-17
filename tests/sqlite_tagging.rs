extern crate diana;

use diana::sqlite_tagging::objects::SqliteObjs;
use diana::tagging::Objects;

#[test]
fn object_adding_result_obj() {
    let objects = SqliteObjs {
        connection: sqlite::open(":memory:").unwrap()
    };
    let result = objects.add_object("https://silverhetch.com");

    assert_eq!("https://silverhetch.com", result.uri())
}

#[test]
fn object_adding_total_size() {
    let objects = SqliteObjs {
        connection: sqlite::open(":memory:").unwrap()
    };
    objects.add_object("https://silverhetch.com");

//    assert_eq!("https://silverhethc.com", object.uri())
}