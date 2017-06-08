// Design Plop - "main.rs"
// Copyright 2017 (c) Jeron Lau

use std::env;

extern crate adi;
use adi::screen::{ GuiButton, Input, Sprite, Window, Transform };

mod model;
use self::model::Model;

struct HandleContext {
	sprite: Sprite,
	held: bool,
	pos: (f32, f32),
}

struct CreatorContext {
	sprite: Sprite,
	pos: (f32, f32),
}

struct Context {
	window: Window,
	models: Vec<Model>,
//	handles: HandleContext,
//	creators: CreatorContext,
	button: GuiButton,
}

const HS : f32 = 0.125;

fn redraw(context: &mut Context) {
//	screen::render(&mut context.window, 60, (0.0, 0.0, 0.0));
}

/*fn update_creator(context: &mut Context, i: usize, j: usize, k: usize, v: &[f32]) {
	let o = i * 8;
	let p = j * 8;
	let x = (v[0 + o] + v[0 + p]) * 0.5;
	let y = (v[1 + o] + v[1 + p]) * 0.5;
	let hm = Matrix::identity()
		.translate(x, y, (v[2 + o] + v[2 + p]) * 0.5)
		.scale(context.window.scalex() * HS,
			context.window.scaley() * HS, 1.0);
	context.creators.matrix(&mut context.window, k, &hm);
	context.creators.context(k).pos = (x, y);
}*/

fn input(context: &mut Context) {
	
//	for i in 0..context.handles.count() {
/*		if context.creators.event(&mut context.window, i, context.event) != -1 {
			new_triangle(context, &V_TRIANGLE);
		}*/
/*		if context.handles.event(&mut context.window, i, context.event) != -1 {
			let l = i / 3;
			let k = i % 3;
			let o = k * 8;
//			context.sprites[l].context(0).vertices[0 + o] =
//				context.handles.context(i).pos.0;
//			context.sprites[l].context(0).vertices[1 + o] =
//				context.handles.context(i).pos.1;
//			let v = context.sprites[l].context(0).vertices;

//			context.sprites[l].vertices(&mut context.window, &v);

			let cv = i % 3;

			let (i, j, k) = match cv {
				0 => (0, 1, 2),
				1 => (1, 2, 0),
				2 => (2, 0, 1),
				_ => panic!("Modulus Failed"),
			};

			update_creator(context, i, j, i + (3 * l), &v);
			update_creator(context, i, k, k + (3 * l), &v);
		}*/
//	}
}

/*fn handle_check(window: &mut Window, handle: &mut HandleContext,
	i: usize, pos: (f32, f32)) -> isize
{
	if handle.context(i).held {
		let hm = Matrix::identity()
			.translate(pos.0, pos.1, 0.0)
			.scale(window.scalex() * HS, window.scaley() * HS, 1.0);
		handle.matrix(window, i, &hm);
		handle.context(i).pos = pos;
		return i as isize;
	}
	-1
}*/

/*fn handle_input(window: &mut Window, handle: &mut HandleContext,
	i: usize, event: Event) -> isize
{
	match event {
		Event::Cursor(x, y) => {
			return handle_check(window, handle, i, (x, y));
		},
		Event::Resize(_, _) => {
			handle.context(i).held = false;
			let (x, y) = handle.context(i).pos;
			let hm = Matrix::identity().translate(x, y, 0.0)
				.scale(window.scalex() * HS,
					window.scaley() * HS, 1.0);
			handle.matrix(window, i, &hm);
		},
		Event::LeftDown(x, y) => {
			let old_pos = handle.context(i).pos;
			let pos = (x, y);
			handle.context(i).held =
				window.scalex() * HS > (old_pos.0 - pos.0).abs() &&
				window.scaley() * HS > (old_pos.1 - pos.1).abs();
			return handle_check(window, handle, i, pos);
		},
		Event::LeftUp(x, y) => {
			let r = handle_check(window, handle, i, (x, y));
			handle.context(i).held = false;
			return r;
		},
		Event::LeaveWindow => {
			handle.context(i).held = false;
		},
		_ => {},
	};
	-1
}*/

/*fn creator_input(window: &mut Window, handle: &mut CreatorContext,
	i: usize, event: Event) -> isize
{
	match event {
		Event::LeftDown(x, y) => {
			let old_pos = handle.context(i).pos;
			let pos = (x, y);
			if window.scalex() * HS > (old_pos.0 - pos.0).abs() &&
				window.scaley() * HS > (old_pos.1 - pos.1).abs()
			{
				return i as isize;
			}
		}
		_ => {},
	}
	-1
}*/

/*fn new_handle(context: &mut Context, vertices: &[f32;24], i: usize) {
	let o = i * 8;
	let hm = Matrix::identity()
		.translate(vertices[0 + o], vertices[1 + o], vertices[2 + o])
		.scale(context.window.scalex() * HS, context.window.scaley() * HS, 1.0);
	context.handles.copy(&mut context.window, &hm, HandleContext {
		held: false, pos: (vertices[0 + o], vertices[1 + o]),
	});
}*/

/*fn new_creator(context: &mut Context, vertices: &[f32;24], i: usize, j: usize) {
	let o = i * 8;
	let p = j * 8;
	let x = (vertices[0 + o] + vertices[0 + p]) * 0.5;
	let y = (vertices[1 + o] + vertices[1 + p]) * 0.5;
	let z = (vertices[2 + o] + vertices[2 + p]) * 0.5;
	let hm = Matrix::identity().translate(x, y, z)
		.scale(context.window.scalex() * HS, context.window.scaley() * HS, 1.0);
	context.creators.copy(&mut context.window, &hm, CreatorContext {
		pos: (x, y),
	});
}*/

fn main() {
	let filename : String = {
		let args = env::args();

		match args.len() {
			0 => panic!("couldn't get arguments"),
			1 => ".".to_string(),
			2 => args.last().unwrap(),
			_ => {
				println!("Too many arguments (MAX=1)!");
				return;
			}
		}
	};

	println!("File: {}", filename);

	// Vertices
	let v_handle = [
		-1.0, -1.0, 0.0, 1.0,	1.0, 1.0, 1.0, 1.0,
		1.0, 1.0, 0.0, 1.0,	1.0, 1.0, 1.0, 1.0,
		1.0, -1.0, 0.0, 1.0,	1.0, 1.0, 1.0, 1.0,

		1.0, 1.0, 0.0, 1.0,	1.0, 1.0, 1.0, 1.0,
		-1.0, -1.0, 0.0, 1.0,	1.0, 1.0, 1.0, 1.0,
		-1.0, 1.0, 0.0, 1.0,	1.0, 1.0, 1.0, 1.0,
	];
	let v_creator = [
		-1.0, -1.0, 0.0, 1.0,	0.5, 0.5, 0.5, 1.0,
		1.0, 1.0, 0.0, 1.0,	0.5, 0.5, 0.5, 1.0,
		1.0, -1.0, 0.0, 1.0,	0.5, 0.5, 0.5, 1.0,

		1.0, 1.0, 0.0, 1.0,	0.5, 0.5, 0.5, 1.0,
		-1.0, -1.0, 0.0, 1.0,	0.5, 0.5, 0.5, 1.0,
		-1.0, 1.0, 0.0, 1.0,	0.5, 0.5, 0.5, 1.0,
	];
	// Open window
	let mut window = Window::create("Design Grizzly",
		include_bytes!("res/logo.ppm"), &[]);

//	let shaders = [
//		screen::shader_color(&mut window),
//		screen::shader_texture(&mut window)
//	];
//	let pipelines = screen::pipeline(&mut window, &shaders);

	let mut context = Context {
		models: Vec::new(),
//		handles: Sprite::colored(&mut window, &v_handle,
//			&pipelines[0], handle_input),
//		creators: Sprite::colored(&mut window, &v_creator,
//			&pipelines[0], creator_input),
		button: GuiButton::create(&mut window, (-1.0, -1.0)),
		window: window,
	};

	context.models.push(Model::create(&mut context.window, filename));

//	new_triangle(&mut context, );

	loop {
		let input = context.window.update();

		match input {
			Input::Back => break,
			Input::Redraw => redraw(&mut context),
			_ => {},
		}

		context.button.update(&mut context.window, input);

		for i in &mut context.models {
			i.update(&mut context.window);
		}

//		input(&mut context);
	}
}
