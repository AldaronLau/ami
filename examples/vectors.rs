// vectors.rs
// Aldaron's Memory Interface ( ami )
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

extern crate ami;

use ami::Vec;

fn main() -> () {
	let mut vec : Vec<i32> = Vec::new();

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
