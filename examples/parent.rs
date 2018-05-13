// Aldaron's Memory Interface
// Copyright (c) 2018 Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// parent.rs

extern crate ami;

use ami::{ Parent, Child, PseudoDrop };

struct TestParent(u32);
struct TestChild(u32);

impl Drop for TestParent {
	fn drop(&mut self) {
		println!("Parent Data Gone {}", self.0);
	}
}

impl PseudoDrop for TestChild {
	type T = TestParent;

	fn pdrop(&mut self, parent: &mut TestParent) {
		println!("Child Data Gone {} / Parent {}", self.0, parent.0);
	}
}

struct Container {
	b: Child<TestParent, TestChild>,
	a: Parent<TestParent, TestChild>,
	d: Child<TestParent, TestChild>,
}

fn main() {
	let mut a = Parent::new(TestParent(45));
	let b = Child::new(&mut a, TestChild(90));
	let d = Child::new(&mut a, TestChild(70));

	let mut c = Container { a, b, d };

	println!("b's ID = {:?}", c.b.id());
	c.b = Child::new(&mut c.a, TestChild(30));
	c.a.data().0 = 12;
	println!("b's ID = {:?}", c.b.id());
	println!("d's ID = {:?}", c.d.id());
}
