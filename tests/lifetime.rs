#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    // fn share(&self) {}
}

/// 有生之年能看到这缺陷被修复吗
#[test]
fn lifetime_defect() {
    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    // foo.share();
    println!("{:?}", loan);
}

struct Ref<'a, T: 'a> {
    r: &'a T,
}

#[test]
fn type_lifetime_compare() {
    let s = &1;
    let r: Ref<&&&&&i32> = Ref { r: &&&&&s };
    let v = r.r;
    println!("v: {}", v);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    fn show(&self) {
        println!("{}, {}", self.x, self.y);
    }
}

/// 使用**再借用**, 可以在一个函数体内多次可变借用一个变量, 只要保证内层借用作用域内不使用外层的借用
#[test]
fn reborrow() {
    let mut p = Point { x: 0, y: 0 };
    p.move_to(1, 2);
    p.show();

    let r = &mut p;
    // 用 &p 报错
    let rr: &Point = &*r;

    println!("second borrow of p: {:?}", rr);
    println!("first borrow of p: {:?}", r);
}

/// 通过修改生命周期声明解决 生命周期过大报错
// struct Interface<'a> {
//     manager: &'a mut Manager<'a>,
// }
struct Interface<'b, 'a: 'b> {
    manager: &'b mut Manager<'a>,
}

// impl<'a> Interface<'a> {
//     pub fn noop(self) {
//         println!("interface consumed");
//     }
// }
impl<'b, 'a: 'b> Interface<'b, 'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    // pub fn get_interface(&'a mut self) -> Interface {
    //     Interface {
    //         manager: &mut self.manager,
    //     }
    // }
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a>
    where 'a: 'b {
        Interface {
            manager: &mut self.manager,
        }
    }
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}

#[test]
fn lifetime_fix() {
    let str: String = "aaaaaa".into();
    let mut list = List {
        manager: Manager { text: str.as_str() },
    };

    // get_interface 可变借用了 list, 这个&'x mut list 的x会随声明而改变
    list.get_interface().noop();

    use_list(&list);
}
