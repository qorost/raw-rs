// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;
use std::ptr;
use std::raw::Slice;
use rawslice::{RawSlice, RawMutSlice};

/// Extension trait for convenience methods on raw pointers
pub trait RawPtrExt<T> {
    /// Converts the pointer into a raw slice.
    fn as_raw_slice(self, len: usize) -> *const [T];

    /// Converts the pointer into a slice.
    unsafe fn as_slice<'a>(self, len: usize) -> &'a [T];

    /// Calculates the offset from a pointer by addition. The offset *must* be in-bounds of
    /// the object, or one-byte-past-the-end.  `count` is in units of T; e.g. a
    /// `count` of 3 represents a pointer offset of `3 * sizeof::<T>()` bytes.
    unsafe fn add(self, count: usize) -> Self;

    /// Calculates the offset from a pointer by subtraction. The offset *must* be in-bounds of
    /// the object, or one-byte-past-the-end.  `count` is in units of T; e.g. a
    /// `count` of 3 represents a pointer offset of `3 * sizeof::<T>()` bytes.
    unsafe fn sub(self, count: usize) -> Self;

    /// Reads the value from `self` and returns it.
    unsafe fn read(self) -> T;

    /// Copies `count * size_of<T>()` many bytes from `self` to `dest`,
    /// assuming that the source and destination *may* overlap.
    unsafe fn copy(self, dest: *mut T, count: usize);

    /// Copies `count * size_of<T>()` many bytes from `self` to `dest`,
    /// assuming that the source and destination *do not* overlap.
    unsafe fn copy_nonoverlapping(self, dest: *mut T, count: usize);
}

/// Extension trait for convenience methods on mutable raw pointers
pub trait RawMutPtrExt<T> {
    /// Converts the pointer into a raw mutable slice.
    fn as_raw_mut_slice(self, len: usize) -> *mut [T];

    /// Converts the pointer into a mutable slice.
    unsafe fn as_mut_slice<'a>(self, len: usize) -> &'a mut [T];

    /// Unsafely overwrite a memory location with the given value without destroying
    /// the old value.
    ///
    /// This operation is unsafe because it does not destroy the previous value
    /// contained at the location `dst`. This could leak allocations or resources,
    /// so care must be taken to previously deallocate the value at `dst`.
    unsafe fn write(self, src: T);

    /// Sets the `count * size_of<T>()` bytes at the address of this pointer to the the given
    /// byte. Good for zeroing out memory.
    unsafe fn write_bytes(self, byte: u8, count: usize);

    /// Swaps the values of `self` and `y`. Note that in contrast to `mem::swap`, `x` and `y`
    /// may point to the same address of memory. Useful for making some operations branchless.
    unsafe fn swap(self, y: *mut T);

    /// Replace the value of the pointer, returning the old value. This is simply
    /// a convenience for calling `mem::replace` with a raw pointer.
    unsafe fn replace(self, src: T) -> T;
}

impl<T> RawPtrExt<T> for *const T {
    fn as_raw_slice(self, len: usize) -> *const [T] {
        unsafe {
            mem::transmute(Slice {
                data: self,
                len: len
            })
        }
    }

    unsafe fn as_slice<'a>(self, len: usize) -> &'a [T] {
        self.as_raw_slice(len).as_slice()
    }

    unsafe fn read(self) -> T {
        ptr::read(self)
    }

    unsafe fn add(self, count: usize) -> Self {
        self.offset(count as isize)
    }

    unsafe fn sub(self, count: usize) -> Self {
        self.offset(-(count as isize))
    }

    unsafe fn copy(self, dest: *mut T, count: usize) {
        ptr::copy(self, dest, count);
    }

    unsafe fn copy_nonoverlapping(self, dest: *mut T, count: usize) {
        ptr::copy_nonoverlapping(self, dest, count);
    }
}

impl<T> RawPtrExt<T> for *mut T {
    fn as_raw_slice(self, len: usize) -> *const [T] {
        (self as *const T).as_raw_slice(len)
    }

    unsafe fn as_slice<'a>(self, len: usize) -> &'a [T] {
        self.as_raw_slice(len).as_slice()
    }

    unsafe fn read(self) -> T {
        ptr::read(self as *const T)
    }

    unsafe fn add(self, count: usize) -> Self {
        self.offset(count as isize)
    }

    unsafe fn sub(self, count: usize) -> Self {
        self.offset(-(count as isize))
    }

    unsafe fn copy(self, dest: *mut T, count: usize) {
        ptr::copy(self, dest, count);
    }

    unsafe fn copy_nonoverlapping(self, dest: *mut T, count: usize) {
        ptr::copy_nonoverlapping(self, dest, count);
    }
}

impl<T> RawMutPtrExt<T> for *mut T {
    fn as_raw_mut_slice(self, len: usize) -> *mut [T] {
        unsafe {
            mem::transmute(Slice {
                data: self as *const T,
                len: len
            })
        }
    }

    unsafe fn as_mut_slice<'a>(self, len: usize) -> &'a mut [T] {
        self.as_raw_mut_slice(len).as_mut_slice()
    }

    unsafe fn write(self, src: T) {
        ptr::write(self, src);
    }

    unsafe fn write_bytes(self, byte: u8, count: usize) {
        ptr::write_bytes(self, byte, count);
    }

    unsafe fn swap(self, y: *mut T) {
        ptr::swap(self, y);
    }

    unsafe fn replace(self, src: T) -> T {
        ptr::replace(self, src)
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_arithmetic() {
        unsafe {
            let mut x = [1,2,3,4];
            let y = x.as_ptr();
            assert_eq!(*y, 1);
            assert_eq!(*y.add(2), 3);
            assert_eq!(*y.add(2).sub(1), 2);

            let y = x.as_mut_ptr();
            assert_eq!(*y, 1);
            assert_eq!(*y.add(2), 3);
            assert_eq!(*y.add(2).sub(1), 2);
        }
    }

    #[test]
    fn test_read_write() {
        unsafe {
            let x = &mut 1 as *mut _;
            assert_eq!(x.read(), 1);
            x.write(2);
            assert_eq!(x.read(), 2);
            x.write_bytes(0, 1);
            assert_eq!(x.read(), 0);
        }
    }

    #[test]
    fn test_copy() {
        unsafe {
            let mut x = [1,2,3,4];
            let y = [5,6,7,8];
            let xptr = x.as_mut_ptr();
            let yptr = y.as_ptr();

            xptr.add(1).copy(xptr, 2);
            assert_eq!(x, [2,3,3,4]);
            yptr.copy_nonoverlapping(xptr, 4);
            assert_eq!(x, y);
        }
    }

    #[test]
    fn test_swap_replace() {
        unsafe {
            let x = &mut 1 as *mut _;
            let y = &mut 2;
            x.swap(y);
            assert_eq!(*x, 2);
            assert_eq!(*y, 1);


            x.replace(3);
            assert_eq!(*x, 3);
        }
    }
}
