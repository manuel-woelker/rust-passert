#![feature(plugin)]
#![plugin(passert_macros)]

extern crate passert;

fn foo() {
    let a = 3;
    let b = -8;
//    passert!(panic!("testing only  x {:?}", 3));
//    let mut helper = passert::PassertHelper::new(1234, "XYZT");
//    helper.add_expression(32, format!("{:?}", a));
    passert!(a + 2 + 3 != -b);
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
