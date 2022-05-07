extern crate libc;

#[link(name = "c_api")]
extern "C" {
    /// test 
    pub fn add(a: i32, b: i32) -> i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn call() {
        let ret = unsafe { add(1, 2) };
        println!("ret: {}", ret);
    }
}
