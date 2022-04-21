use bytes::Buf;

#[test]
fn lib_bytes() {
    let mut buf = &b"hello"[..];
    let mut buf_bytes: Buf = buf;
    println!(buf_bytes);
}

#[test]
fn string_to_str() {
    let k = String::from("hello");
    let r: &str = &k;
    assert_eq!(k.as_str(), r);
}