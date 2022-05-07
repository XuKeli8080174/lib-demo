#![allow(dead_code)]
#![allow(unused_variables)]

use std::{marker::PhantomPinned, pin::Pin, ptr::NonNull};

#[derive(Debug)]
struct Foo {
    foo: u64,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("drop invoke for Foo: {:?}, foo: {}", self, self.foo);
    }
}

#[test]
fn drop_test() {
    let mut v = vec![Foo { foo: 1 }, Foo { foo: 2 }, Foo { foo: 3 }];
    v[2] = Foo { foo: 1145141919810 };
}

/// 使用Pin创建自引用字段结构体, 代价是不能用std::mem的方法改变值的地址
struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}
impl Unmovable {
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            // 只有在数据到位时，才创建指针，否则数据会在开始之前就被转移所有权
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);

        let slice = NonNull::from(&boxed.data);
        // 这里其实安全的，因为修改一个字段不会转移整个结构体的所有权
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }
}

#[test]
fn r() {
    let _a = kvs::Arc::new(1);
}
