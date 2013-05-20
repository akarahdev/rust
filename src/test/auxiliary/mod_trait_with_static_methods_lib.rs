// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use sub_foo::Foo;
pub use Baz = self::Bar;

pub trait Bar {
    pub fn bar() -> Self;
}

impl Bar for int {
    pub fn bar() -> int { 84 }
}

pub mod sub_foo {
    pub trait Foo {
        pub fn foo() -> Self;
    }

    impl Foo for int {
        pub fn foo() -> int { 42 }
    }

}
