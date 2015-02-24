pub fn test<T: Foo>(_: T) {}

pub trait Foo {
    fn bar(&self);
}
