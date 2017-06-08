// Design Plop - "model/file.rs"
// Copyright 2017 (c) Jeron Lau

use std::fs;
use std::fs::File;
use std::fs::Metadata;
use std::io::BufReader;
use std::io::prelude::*;
use std::time::SystemTime;

pub fn parse(filename: String) -> (Vec<f32>, SystemTime) {
	let file = File::open(filename.clone());
	let mut contents = String::new();

	file.unwrap().read_to_string(&mut contents);

	// Step 1: Find and remove '['.
	loop {
		match contents.remove(0) {
			' ' | '\t' | '\n' => continue,
			'[' => break,
			_ => panic!("Err: File doesn't start with `[`!"),
		}
	}

	// Step 2: Find and remove ']'
	loop {
		match contents.pop().unwrap() {
			' ' | '\t' | '\n' => continue,
			']' => break,
			_ => panic!("Err: File doesn't end with `]`!"),
		}
	}

	// Step 3: Split by whitespace and commas.
	let mut chunks = contents.split(|c| match c {
		' ' | '\t' | '\n' | ',' => true,
		_ => false
	});

	// Step 4: Convert the values from strings to Vec<f32>.
	let mut converted = Vec::new();

	while let Some(chunk) = chunks.next() {
		let mut non_separated : Vec<&str> = chunk.split(',').collect();
		non_separated.retain(|&x| x != "");

		for i in non_separated {
			println!("c: {}", i);

			let n : f32 = i.parse::<f32>().unwrap();

			converted.push(n);
		}
	}

	println!("contents: {}", contents);

	(converted, fs::metadata(filename).unwrap().modified().unwrap())
}

pub fn changed(file: String, since: SystemTime) -> bool {
	let metadata = fs::metadata(file).unwrap();

	if let Ok(time) = metadata.modified() {
		since != time
	} else {
		panic!("Not supported on this platform");
	}
}
