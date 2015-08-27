#![feature(plugin)]
#![plugin(passert_macros)]

extern crate passert;

#[test]
#[should_panic]
fn it_works() {
    let a = 3;
    let b = 4;
    passert!(a + 2 + 3 == -b);
}
