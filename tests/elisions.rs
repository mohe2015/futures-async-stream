#![warn(rust_2018_idioms, single_use_lifetimes)]
#![feature(generators)]

use futures_async_stream::stream;

pub struct Ref<'a, T>(&'a T);

#[stream(item = i32)]
pub async fn references(x: &i32) {
    yield *x;
}

#[stream(item = i32)]
pub async fn new_types(x: Ref<'_, i32>) {
    yield *x.0;
}

pub struct Foo(i32);

impl Foo {
    #[stream(item = &i32)]
    pub async fn foo(&self) {
        yield &self.0
    }
}

#[stream(item = &i32)]
pub async fn single_ref(x: &i32) {
    yield x
}

#[stream(item = ())]
pub async fn multi_ref<T>(_x: &T, _y: &i32) {
    yield
}

#[stream(item = ())]
pub async fn pat_ref<T>(_x: &T, _y: (&i32, &i8)) {
    yield
}

#[allow(single_use_lifetimes)]
#[stream(item = ())]
pub async fn check_for_name_collision<'_async0, T>(_x: &T, _y: &'_async0 i32) {
    yield
}
