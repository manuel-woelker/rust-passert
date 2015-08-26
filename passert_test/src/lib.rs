#![feature(plugin)]
#![plugin(passert_macros)]

fn foo() {
    let a = 3;
    let b = 4;
//    passert!(panic!("testing only  x {:?}", 3));
    passert!(a+1 == b);
//    let msg = String::from("foobar");
//    passert!(panic!("foo"));
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
