// Design Plop - "model/mod.rs"
// Copyright 2017 (c) Jeron Lau

use std::time::SystemTime;

use adi::screen::{ Sprite, Style, Window };

mod file;

pub struct Model {
	sprite: Sprite,
	vertices: Vec<f32>,
	filename: String,
	modified: SystemTime,
}

const V_TRIANGLE : [f32;24] = [
	// Front Side
	-0.5,  0.5, 0., 1.0,	1.0, 1.0, 1.0, 1.0,
	 0.5,  0.5, 0., 1.0,	1.0, 1.0, 1.0, 1.0,
	 0.0, -0.5, 0., 1.0,	1.0, 1.0, 1.0, 1.0,
];

impl Model {
	pub fn create(window: &mut Window, filename: String) -> Model {
		let (vertices, modified) = if filename == "." {
			let mut v = Vec::new();

			v.extend_from_slice(&V_TRIANGLE);

			(v, SystemTime::now())
		} else {
			file::parse(filename.clone())
		};

/*		new_handle(context, vertices, 0);
		new_handle(context, vertices, 1);
		new_handle(context, vertices, 2);

		new_creator(context, vertices, 0, 1);
		new_creator(context, vertices, 1, 2);
		new_creator(context, vertices, 2, 0);*/

		let style = Style::create().gradient();

		Model {
			sprite: Sprite::create(window, &vertices, style, 1),
			vertices: vertices,
			filename: filename,
			modified: modified,
		}
	}

	pub fn update(&mut self, window: &mut Window) {
		if self.filename != "." &&
			file::changed(self.filename.clone(), self.modified)
		{
			let (vertices, modified) =
				file::parse(self.filename.clone());

			self.vertices = vertices;
			self.modified = modified;

			self.sprite.vertices(window, &self.vertices[..]);
			println!("update");
		}
	}
}
