pub trait Source<T> {
    fn fetch(&self) -> &T;
}

pub struct ConstSource<'a, T> {
    pub value: &'a T
}

impl<'a, T> Source<T> for ConstSource<'a, T> {
    fn fetch(&self) -> &T {
        return self.value;
    }
}