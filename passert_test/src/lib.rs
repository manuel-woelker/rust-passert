#![feature(plugin)]
#![plugin(passert)]

fn foo() {
    let a = 3;
    let b = 4;
//    passert!(a == b);
    passert!(panic!("foo"));
}

#[test]
fn it_works() {
    foo();
//    let a = 3;
//    let b = 4;
//    passert!(a == b);
//    assert_eq!(rn!(MMXV), 2014);
//    assert_eq!(rn!(MMXV), 2014);
}
