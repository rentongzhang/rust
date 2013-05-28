// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// When denying at the crate level, be sure to not get random warnings from the
// injected intrinsics by the compiler.
#[deny(missing_doc)];

struct Foo {
    a: int,
    priv b: int,
    pub c: int, // doesn't matter, Foo is private
}

pub struct PubFoo { //~ ERROR: missing documentation
    a: int,      //~ ERROR: missing documentation
    priv b: int,
    pub c: int,  //~ ERROR: missing documentation
}

#[allow(missing_doc)]
pub struct PubFoo2 {
    a: int,
    pub c: int,
}

/// dox
pub fn foo() {}
pub fn foo2() {} //~ ERROR: missing documentation
fn foo3() {}
#[allow(missing_doc)] pub fn foo4() {}

/// dox
pub trait A {}
trait B {}
pub trait C {} //~ ERROR: missing documentation
#[allow(missing_doc)] pub trait D {}

trait Bar {
    /// dox
    pub fn foo();
    fn foo2(); //~ ERROR: missing documentation
    pub fn foo3(); //~ ERROR: missing documentation
}

impl Foo {
    pub fn foo() {} //~ ERROR: missing documentation
    /// dox
    pub fn foo1() {}
    fn foo2() {}
    #[allow(missing_doc)] pub fn foo3() {}
}

#[allow(missing_doc)]
trait F {
    pub fn a();
    fn b(&self);
}

// should need to redefine documentation for implementations of traits
impl F for Foo {
    pub fn a() {}
    fn b(&self) {}
}

fn main() {}
