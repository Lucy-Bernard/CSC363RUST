#![allow(dead_code)]

fn string_to_vector (s : &str) -> Vec<u8> {
  s.bytes ().collect::<Vec<_>> ()
}

fn vector_to_string (v : &[u8]) -> String {
  String::from_utf8_lossy (v).into_owned ()
}


fn main() {

    let v1: Vec<u8> = vec![104,105];
    dbg! (string_to_vector("hi"));
    dbg! (vector_to_string(&v1));
}
