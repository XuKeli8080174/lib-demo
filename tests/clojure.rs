#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;

#[derive(Debug)]
struct Foo;

impl Foo {
    fn len(&self) -> usize {
        return 1;       
    }
    fn mut_consume(&mut self) {
        println!("consume Foo, {:#?}", self);
    }
}

fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

#[test]
fn once() {
    // let x = vec![1, 2, 3];
    let x = Foo{};
    fn_once(|z|{/*x.mut_consume();*/z == x.len()});
    println!("{:?}", x);

    // let arr = [Foo{}; 100];
}


#[test]
fn cell() {
    let a = RefCell::new(String::from("aaaaaaaaaaaaa"));
    let ab = a.borrow_mut();
    println!("{:?}", ab);
    drop(ab);
    let ac = a.borrow_mut();
    println!("{:?}", ac);
}

fn consume<T>(_t: T) {

}
