pub trait Tags {
    fn add_tag(&self) -> &Tag;
}

pub trait Tag {
    fn name(&self) -> &str;
}

pub trait Objects {
    fn add_object(&self, uri: &str) -> &Object;
}

pub trait Object {
    fn uri(&self) -> &str;
}