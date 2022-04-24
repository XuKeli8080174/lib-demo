#[test]
fn lib_bytes() {
    let mut _buf = &b"hello"[..];
}

#[test]
fn string_to_str() {
    let k = String::from("hello");
    let r: &str = &k;
    assert_eq!(k.as_str(), r);
}
