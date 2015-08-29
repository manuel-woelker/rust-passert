#![feature(plugin)]
#![plugin(passert_macros)]

extern crate passert;

#[cfg(test)]
use std::cmp;
#[cfg(test)]
use std::ascii::AsciiExt;

#[test]
fn literal_true() {
    passert!(true);
}

#[test]
#[should_panic(expected = "Assertion failed: false")]
fn literal_false() {
    passert!(false);
}

#[test]
fn binding_true() {
    let t = true;
    passert!(t);
}

#[test]
#[should_panic(expected = "Assertion failed: f")]
fn binding_false() {
    let f = false;
    passert!(f);
}

#[test]
fn unary_expression_true() {
    let f = false;
    passert!(!f);
}

#[test]
#[should_panic(expected = "Assertion failed: !t")]
fn unary_expression_false() {
    let t = true;
    passert!(!t);
}

#[test]
fn binary_expression_true() {
    let a = 3;
    let b = -8;
    passert!(a + 2 + 3 == -b);
}

#[test]
#[should_panic(expected = "Assertion failed: a + 2 + 3 == -b")]
fn binary_expression_false() {
    let a = 3;
    let b = 4;
    passert!(a + 2 + 3 == -b);
}

#[test]
fn macro_expression_true() {
    passert!(format!("{}{}", "foo", "bar") == "foobar");
}

#[test]
#[should_panic(expected = "format!(\"{}{}\", \"foo\", \"bar\") == \"fizzbuzz\"")]
fn macro_expression_false() {
    passert!(format!("{}{}", "foo", "bar") == "fizzbuzz");
}

#[test]
fn paren_expression_true() {
    let a = 3;
    let b = -8;
    passert!((a + 2) + 3 == -b);
}

#[test]
#[should_panic(expected = "Assertion failed: (a + 2) + 3 == -b")]
fn paren_expression_false() {
    let a = 3;
    let b = 4;
    passert!((a + 2) + 3 == -b);
}

#[test]
fn call_expression_true() {
    let a = 1;
    let b = 2;
    passert!(cmp::max(a, b) == 2);
}

#[test]
#[should_panic(expected = "Assertion failed: cmp::max(a, b) == 3")]
fn call_expression_false() {
    let a = 1;
    let b = 2;
    passert!(cmp::max(a, b) == 3);
}

#[test]
fn method_call_expression_true() {
    let str = "foo";
    passert!(str.to_ascii_uppercase() == "FOO");
}

#[test]
#[should_panic(expected = "Assertion failed: str.to_ascii_uppercase() == \"BAR\"")]
fn method_call_expression_false() {
    let str = "foo";
    passert!(str.to_ascii_uppercase() == "BAR");
}
