// Aldaron's Memory Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// examples/vectors.rs

extern crate ami;

fn main() {
	let mut vec : ami::Vec<i32> = ami::Vec::new();

	println!("Empty Vec: {}", vec);
	vec.push(12);
	println!("Pushed 12: {}", vec);
	vec.push(102);
	println!("Pushed 102: {}", vec);
	println!("vec[1] = {}", vec[1]);
	println!("Pop: {}", vec.pop().unwrap());
	println!("Vector: {}", vec);
	println!("vec[0] = {}", vec[0]);
}
