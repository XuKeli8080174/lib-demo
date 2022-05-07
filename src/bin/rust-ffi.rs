use kvs::ffi_test::add;

fn main() {
    let ret = unsafe { add(1, 2) };
    println!("ffi add ret: {}", ret);
}
