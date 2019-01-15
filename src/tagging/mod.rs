pub trait Tags {
    fn iterator(&self) -> Box<Iterator<Item=Object>>;
    fn add_tag(&self) -> Box<Tag>;
}

pub trait Tag {
    fn name(&self) -> &str;
}

pub trait Objects {
    fn add_object(&self, uri: &str) -> Box<Object>;
}

pub trait Object {
    fn uri(&self) -> &str;
}