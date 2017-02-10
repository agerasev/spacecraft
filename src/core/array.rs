use num::*;
use la::vec::*;

use core::map::*;

pub struct Array<T> {
	data_: Vec<T>,
	size_: vec3i,
}

impl<T> Array<T> {
	pub fn new<F>(size: vec3i, mut f: F) -> Array<T> where F: FnMut(vec3i) -> T {
		assert!(size.gt_(vec3::zero()).all());
		let mut v = Vec::new();
		let vec_len = (8*size[0] as usize)*(size[1] as usize)*(size[2] as usize);
		v.reserve(vec_len);
		for iv in (-size).iter_to(size) {
			v.push(f(iv));
		}
		assert_eq!(v.len(), vec_len);
		Array { data_: v, size_: size }
	}

	fn itov(&self, i: usize) -> vec3i {
		let s = self.size_;
		vec3i::from([
			(i % (2*s[0] as usize)) as i32 - s[0],
			((i/(2*s[0] as usize)) % (2*s[1] as usize)) as i32 - s[1],
			(i/((2*s[0] as usize)*(2*s[1] as usize))) as i32 - s[2],
		])
	}

	fn vtoi(&self, v: vec3i) -> usize {
		let s = self.size_;
		(((v[2] + s[2])*(2*s[1]) + (v[1] + s[1])) as usize)*(2*s[0] as usize) + (v[0] + s[0]) as usize
	}
}

impl<T> Array<T> where T: Copy {
	pub unsafe fn get_unchecked(&self, v: vec3i) -> T {
		let i = self.vtoi(v);
		*self.data_.get_unchecked(i)
	}

	pub unsafe fn set_unchecked(&mut self, v: vec3i, b: T) {
		let i = self.vtoi(v);
		*self.data_.get_unchecked_mut(i) = b;
	}
}

impl<T> Array<T> {
	pub unsafe fn get_ref_unchecked(&self, v: vec3i) -> &T {
		let i = self.vtoi(v);
		self.data_.get_unchecked(i)
	}

	pub unsafe fn get_ref_mut_unchecked(&mut self, v: vec3i) -> &mut T {
		let i = self.vtoi(v);
		self.data_.get_unchecked_mut(i)
	}
}

impl<T> Size3 for Array<T> {
	fn size(&self) -> vec3i {
		self.size_
	}
}

impl<T> Map<T> for Array<T> where T: Copy {
	fn get(&self, v: vec3i) -> T {
		assert!(self.inside(v));
		unsafe { self.get_unchecked(v) }
	}

	fn set(&mut self, v: vec3i, b: T) {
		assert!(self.inside(v));
		unsafe { self.set_unchecked(v, b); }
	}
}

impl<T> MapRef<T> for Array<T> {
	fn get_ref(&self, v: vec3i) -> &T {
		assert!(self.inside(v));
		unsafe { self.get_ref_unchecked(v) }
	}

	fn get_ref_mut(&mut self, v: vec3i) -> &mut T {
		assert!(self.inside(v));
		unsafe { self.get_ref_mut_unchecked(v) }
	}
}