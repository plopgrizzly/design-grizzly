extern crate aldaron;

use aldaron::screen;
use aldaron::screen::{ Event, Sprite, Window, Matrix };
use aldaron::screen::gui::{ Button };

struct SpriteContext {
	vertices: [f32;24],
}

struct HandleContext {
	held: bool,
	pos: (f32, f32),
}

struct Context {
	window: Window,
	event: Event,
	triangle: Sprite<SpriteContext>,
	handles: Sprite<HandleContext>,
	button: Sprite<Button>,
}

const HS : f32 = 0.125;

fn draw(context: &mut Context) {
	screen::render(&mut context.window, 60, (0.0, 0.0, 0.0));
}

fn input(context: &mut Context) {
	match context.event {
		Event::None => draw(context),
		Event::KeyDown(p, s) => println!("press \"{}\", {}", p, s),
		Event::KeyUp(p, s) => println!("release \"{}\", {}", p, s),
		Event::KeyRepeat(p, s) => println!("repeat \"{}\", {}", p, s),
		Event::Resize(w, h) => println!("Resize({}, {})", w, h),
		Event::MiddleDown(x, y) => println!("Middle Down ({}, {})", x, y),
		Event::MiddleUp(x, y) => println!("Middle Up ({}, {})", x, y),
		Event::RightDown(x, y) => println!("Right Down ({}, {})", x, y),
		Event::RightUp(x, y) => println!("Right Up ({}, {})", x, y),
		Event::ScrollUp(x, y) => println!("Scroll Up ({}, {})", x, y),
		Event::ScrollDown(x, y) => println!("Scroll Down ({}, {})", x, y),
		Event::ScrollRight(x, y) => println!("Scroll Right ({}, {})", x, y),
		Event::ScrollLeft(x, y) => println!("Scroll Left ({}, {})", x, y),
		Event::EnterWindow => println!("Enter Window"),
		Event::LeaveWindow => println!("Leave Window"),
		Event::Resume => println!("Resume ( Gain Focus )"),
		Event::Pause => println!("Pause ( Lose Focus )"),
		_ => {},
	};
	context.button.run(&mut context.window, context.event);
	for i in 0..context.handles.count() {
		match context.handles.event(&mut context.window, i, context.event) {
			0 => {
				let j = i / 3;
				let k = i % 3;
				let o = k * 8;
				context.triangle.context(j).vertices[0 + o] =
					context.handles.context(i).pos.0;
				context.triangle.context(j).vertices[1 + o] =
					context.handles.context(i).pos.1;
				let v = context.triangle.context(j).vertices;

				context.triangle.vertices(&mut context.window, &v);
			},
			_ => {},
		}
	}
}

fn logo_input(_: &mut Window, _: &mut Sprite<SpriteContext>, _: usize, _: Event)
	 -> isize
{
	-1
}

fn handle_check(window: &mut Window, handle: &mut Sprite<HandleContext>,
	i: usize, pos: (f32, f32)) -> isize
{
	if handle.context(i).held {
		let hm = Matrix::identity()
			.translate(pos.0, pos.1, 0.0)
			.scale(window.scalex() * HS, window.scaley() * HS, 1.0);
		handle.matrix(window, i, &hm);
		handle.context(i).pos = pos;
		return 0;
	}
	-1
}

fn handle_input(window: &mut Window, handle: &mut Sprite<HandleContext>,
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
}

fn new_handle(context: &mut Context, vertices: [f32;24], i: usize) {
	let o = i * 8;
	let hm = Matrix::identity()
		.translate(vertices[0 + o], vertices[1 + o], vertices[2 + o])
		.scale(context.window.scalex() * HS, context.window.scaley() * HS, 1.0);
	context.handles.copy(&mut context.window, &hm, HandleContext {
		held: false, pos: (vertices[0 + o], vertices[1 + o]),
	});
}

fn main() {
	// Vertices
	let v_triangle = [
		// Front Side
		-0.5,  0.5, 0., 1.0,	1.0, 0.0, 0.0, 1.0,
		 0.5,  0.5, 0., 1.0,	0.0, 1.0, 0.0, 1.0,
		 0.0, -0.5, 0., 1.0,	0.0, 0.0, 1.0, 1.0,
	];
	let v_square = [
		-1.0, -1.0, 0.0, 1.0,	1.0, 1.0, 1.0, 1.0,
		1.0, 1.0, 0.0, 1.0,	1.0, 1.0, 1.0, 1.0,
		1.0, -1.0, 0.0, 1.0,	1.0, 1.0, 1.0, 1.0,

		1.0, 1.0, 0.0, 1.0,	1.0, 1.0, 1.0, 1.0,
		-1.0, -1.0, 0.0, 1.0,	1.0, 1.0, 1.0, 1.0,
		-1.0, 1.0, 0.0, 1.0,	1.0, 1.0, 1.0, 1.0,
	];
	// Matrices
	let im = screen::Matrix::identity();
	// Open window
	let mut window = screen::init("Design Grizzly",
		include_bytes!("res/logo.ppm"), false);

	let shaders = [
		screen::shader_color(&mut window),
		screen::shader_texture(&mut window)
	];
	let pipelines = screen::pipeline(&mut window, &shaders);

	let mut triangle = Sprite::colored(&mut window, &v_triangle,
		&pipelines[0], logo_input);
	triangle.copy(&mut window, &im, SpriteContext { vertices: v_triangle });

	let mut context = Context {
		triangle: triangle,
		handles: Sprite::colored(&mut window, &v_square,
			&pipelines[0], handle_input),
		button: Button::add(&mut window, &pipelines[1], (-1.0, -1.0)),
		window: window,
		event: Event::None,
	};

	new_handle(&mut context, v_triangle, 0);
	new_handle(&mut context, v_triangle, 1);
	new_handle(&mut context, v_triangle, 2);

	while screen::running(&mut context.window, &mut context.event) {
		input(&mut context);
	}
	screen::cleanup(&mut context.window);
}
