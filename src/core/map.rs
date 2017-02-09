use la::vec::*;

pub trait Size3 {
	fn size(&self) -> vec3i;
	fn inside(&self, v: vec3i) -> bool {
		(v.ge_(-self.size()) & v.lt_(self.size())).all()
	}
}

pub trait Map<T: Copy> {
	fn get(&self, v: vec3i) -> T;
	fn set(&mut self, v: vec3i, t: T);
}

pub trait MapRef<T> {
	fn get_ref(&self, v: vec3i) -> &T;
	fn get_ref_mut(&mut self, v: vec3i) -> &mut T;
}


macro_rules! derive_size3 {
	($Struct:ident, $field:ident) => (
		impl Size3 for $Struct {
			#[inline]
			fn size(&self) -> vec3i {
				self.$field.size()
			}
		}
	)
}


macro_rules! derive_map {
	($Struct:ident, $field:ident, $Output:ident) => (
		impl Map<$Output> for $Struct {
			#[inline]
			fn get(&self, v: vec3i) -> $Output {
				self.$field.get(v)
			}

			#[inline]
			fn set(&mut self, v: vec3i, t: $Output) {
				self.$field.set(v, t);
			}
		}
	)
}

macro_rules! derive_map_ref {
	($Struct:ident, $field:ident, $Output:ident) => (
		impl MapRef<$Output> for $Struct {
			#[inline]
			fn get_ref(&self, v: vec3i) -> &$Output {
				self.$field.get_ref(v)
			}

			#[inline]
			fn get_ref_mut(&mut self, v: vec3i) -> &mut $Output {
				self.$field.get_ref_mut(v)
			}
		}
	)
}
