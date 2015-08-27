# rust-passert

[![Travis Build Status](https://travis-ci.org/manuel-woelker/rust-passert.svg)](https://travis-ci.org/manuel-woelker/rust-passert)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Pretty/Power assertion macros for rust

## Overview

An assert macro in the spirit of
[Spock/Groovy assertions]
(http://docs.groovy-lang.org/latest/html/documentation/core-testing-guide.html#_power_assertions) which evaluates and prints subexpressions,
simplifying failure analysis

## Example

```rust
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
```

Output


```
running 1 test
Assertion failed:
a + 2 + 3 == -b
| |   |   |  ||
3 5   8   |  |4
          |  -4
          false
thread 'it_works' panicked at 'Assertion failed: a + 2 + 3 == -b', src/lib.rs:11

```
