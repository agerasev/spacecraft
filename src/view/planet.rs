use num::*;
use la::vec::*;
use la::mat::*;

use core::pos::*;
use core::ori::*;
use core::block::*;
use core::map::{Map, Size3};
use core::planet::Planet as CorePlanet;

use view::draw::Draw;
use view::engine::Engine;
use view::model::*;

use gl4u::gl::types::*;
use gl4u::buffer::Buffer;
use gl4u::pass::Prim;
use gl4u::error::Error;

pub struct Planet {
	planet: CorePlanet,

	vertex: Buffer,
	color: Buffer,
	tex_pos: Buffer,
	occlusion: Buffer,

	bufsize: i32,
	dirty: bool,
}

derive_pos_mut!(Planet, planet);
derive_ori_mut!(Planet, planet);

impl_model!(Planet);

derive_size3!(Planet, planet);

impl Map<Block> for Planet {
	#[inline]
	fn get(&self, v: vec3i) -> Block {
		self.planet.get(v)
	}

	#[inline]
	fn set(&mut self, v: vec3i, b: Block) {
		self.planet.set(v, b);
		self.dirty = true;
	}
}

impl Planet {
	pub fn new(chunk_size: i32, chunk_count: i32) -> Self {
		Planet {
			planet: CorePlanet::new(chunk_size, chunk_count),
			vertex: Buffer::new(3),
			color: Buffer::new(3),
			tex_pos: Buffer::new(2),
			occlusion: Buffer::new(4),
			bufsize: 0,
			dirty: true
		}
	}

	pub fn update(&mut self) {
		let mut size: i32 = 0;
		let mut vert = Vec::<GLfloat>::new();
		let mut cols = Vec::<GLfloat>::new();
		let mut texp = Vec::<GLfloat>::new();
		let mut occl = Vec::<GLfloat>::new();


		{
			let mut add_vertex = |vpos: vec3i, tpos: vec2i, vcol: vec3d| {
				let v: vec3d = vec3d::from_(vpos)*SIZE;
				let c = vcol;
				let t = tpos;
				for i in 0..3 {
					vert.push(v[i] as GLfloat);
					cols.push(c[i] as GLfloat);
				}
				for i in 0..2 {
					texp.push(t[i] as GLfloat);
				}
			};

			let mut add_edge = |pos: vec3i, dir: vec3i| {
				let s = dir.dot([1, -1, 1].into());

				let (ix, iy) = if dir[0] == 0 { (0, if dir[1] == 0 { 1 } else { 2 } ) } else { (1, 2) };
				let (bx, by) = (vec3i::map(|i| (i == ix) as i32), vec3i::map(|i| (i == iy) as i32));

				let col: vec3d = [0.2, 0.8, 0.2].into();

				let mut voccl = vec4d::new();
				for i in 0..4 {
					let d2 = vec2i::from([if i < 2 { 1 } else { -1 }, if i == 0 || i == 3 { 1 } else { -1 }]);
					let (dx, dy) = (bx*d2[0], by*d2[1]*s);
					let d3 = dir + dx + dy;
					let vpos = pos + vec3i::map(|i| (d3[i] > 0) as i32);
					let tpos = vec2i::map(|i| (d2[i] > 0) as i32);
					let mut near = [0; 4];
					for k in 0..4 {
						near[k as usize] = (self.get(pos + dir + dx*(k % 2) + dy*(k/2)) == VOID) as u32;
					}
					if near[1] == 0 || near[2] == 0 { near[3] = 0; }
					voccl[i] = match near[0] + near[1] + near[2] + near[3] {
						0 => 0.0,
						1 => 1.0/3.0,
						2 | 3 => 2.0/3.0,
						4 => 1.0,
						_ => 0.0,
					};
					add_vertex(vpos, tpos, col);
				}
				for _ in 0..4 {
					for i in 0..4 {
						occl.push(voccl[i] as GLfloat);
					}
				}
				size += 1;
			};

			for iz in -self.size()[2]..self.size()[2] {
				for iy in -self.size()[1]..self.size()[1] {
					for ix in -self.size()[0]..self.size()[0] {
						let dirs = [
							vec3i::from([1, 0, 0]),
							vec3i::from([0, 1, 0]),
							vec3i::from([0, 0, 1]),
							vec3i::from([-1, 0, 0]),
							vec3i::from([0, -1, 0]),
							vec3i::from([0, 0, -1]),
						];
						let pos = vec3i::from([ix, iy, iz]);
						for dir in dirs.iter() {
							if self.get(pos) != VOID {
								if self.get(pos + *dir) == VOID {
									add_edge(pos, *dir);
								}
							}
						}
					}
				}
			}
		}

		self.bufsize = size;
		if size > 0 {
			self.vertex.load_float(&vert);
			self.color.load_float(&cols);
			self.tex_pos.load_float(&texp);
			self.occlusion.load_float(&occl);
		} else {
			// self.vertex.clear();
			// self.color.clear();
		}

		self.dirty = false;
	}
}

impl Draw for Planet {
	fn draw(&mut self, engine: &mut Engine) -> Result<(), Error> {
		if self.dirty { self.update(); }
		if self.bufsize > 0 {
			let mut pass = engine.use_program("array").unwrap();
			pass = engine.bind_camera(pass);
			pass = try!(pass.uniform_matrix("model", self.model().data()));
			pass = try!(pass.attribute("position", &self.vertex));
			pass = try!(pass.attribute("color", &self.color));
			pass = try!(pass.attribute("tex_pos", &self.tex_pos));
			pass = try!(pass.attribute("occlusion", &self.occlusion));
			pass.primitive(Prim::Quads).range(0, self.bufsize*4).draw()
		} else {
			Ok(())
		}
	}
}