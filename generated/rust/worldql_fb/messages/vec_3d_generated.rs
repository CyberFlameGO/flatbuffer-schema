// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
// struct Vec3d, aligned to 8
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vec3d(pub [u8; 24]);
impl Default for Vec3d { 
  fn default() -> Self { 
    Self([0; 24])
  }
}
impl std::fmt::Debug for Vec3d {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("Vec3d")
      .field("x", &self.x())
      .field("y", &self.y())
      .field("z", &self.z())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Vec3d {}
impl flatbuffers::SafeSliceAccess for Vec3d {}
impl<'a> flatbuffers::Follow<'a> for Vec3d {
  type Inner = &'a Vec3d;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Vec3d>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Vec3d {
  type Inner = &'a Vec3d;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Vec3d>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Vec3d {
    type Output = Vec3d;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Vec3d as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Vec3d {
    type Output = Vec3d;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Vec3d as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for Vec3d {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}
impl<'a> Vec3d {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    x: f64,
    y: f64,
    z: f64,
  ) -> Self {
    let mut s = Self([0; 24]);
    s.set_x(x);
    s.set_y(y);
    s.set_z(z);
    s
  }

  pub fn x(&self) -> f64 {
    let mut mem = core::mem::MaybeUninit::<f64>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<f64>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_x(&mut self, x: f64) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f64 as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<f64>(),
      );
    }
  }

  pub fn y(&self) -> f64 {
    let mut mem = core::mem::MaybeUninit::<f64>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[8..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<f64>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_y(&mut self, x: f64) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f64 as *const u8,
        self.0[8..].as_mut_ptr(),
        core::mem::size_of::<f64>(),
      );
    }
  }

  pub fn z(&self) -> f64 {
    let mut mem = core::mem::MaybeUninit::<f64>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[16..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<f64>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_z(&mut self, x: f64) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f64 as *const u8,
        self.0[16..].as_mut_ptr(),
        core::mem::size_of::<f64>(),
      );
    }
  }

}
