// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rawptr::{RawPtrExt, RawMutPtrExt};

/// Extension trait for non-mutating operations on raw slices.
pub trait RawSlice<T>: Copy + Sized {
    /// Converts the rawslice into a slice.
    unsafe fn as_slice<'a>(self) -> &'a [T];

    /// Gets the length of the rawslice.
    fn len(self) -> usize {
        unsafe { self.as_slice().len() }
    }

    /// Converts the rawslice into a rawptr.
    fn as_ptr(self) -> *const T {
        unsafe { self.as_slice().as_ptr() }
    }

    /// Reads the data at the given index and interprets it as a value of T.
    /// This does not move the value out, and ignores the length of the raw slice.
    unsafe fn read(self, index: usize) -> T {
        self.as_ptr().add(index).read()
    }

    /// Gets a reference to the element at the given index.
    unsafe fn get<'a>(self, index: usize) -> &'a T {
        &*self.as_ptr().add(index)
    }

    /// Gets a subslice of this one.
    unsafe fn slice(self, from: usize, to: usize) -> Self;

    /// Gets a subslice from 0 to `to`.
    fn slice_to(self, to: usize) -> Self {
        unsafe { self.slice(0, to) }
    }

    /// Gets a subslice from `from` to the end of the slice.
    unsafe fn slice_from(self, from: usize) -> Self {
        self.slice(from, self.len())
    }
}


/// Extension trait for mutating operations on raw slices.
pub trait RawMutSlice<T> : RawSlice<T> + Sized {
    /// Converts the rawslice into a mutable slice.
    unsafe fn as_mut_slice<'a>(self) -> &'a mut[T];

    /// Converts the rawslice into a mutable rawptr.
    fn as_mut_ptr(self) -> *mut T;

    /// Writes a value to the given index without reading or destroying whatever
    /// data might exist at that index. Appropriate for initializing unitialized data.
    /// Ignores the length of the raw slice.
    unsafe fn write(self, index: usize, val: T);

    /// Sets every byte in the slice to to the given one, without reading or destroying whatever
    /// data might have been contained. Can be used to zero memory out.
    unsafe fn write_bytes(self, byte: u8);

    /// Copies the contents of the given rawslice into this one, assuming that they might
    /// have overlapping regions of memory. Uses from.len() to determine the length of the
    /// copied data, but does not consider the target's length.
    unsafe fn copy(self, from: *const[T]);

    /// Copies the contents of the given rawslice into this one, assuming they don't have any
    /// overlapping memory. Uses `from.len()` to determine the length copied data, but does
    /// not consider the target's length.
    unsafe fn copy_nonoverlapping(self, from: *const[T]);

    /// Gets a mutable reference to the value at the given index.
    unsafe fn get_mut<'a>(self, index: usize) -> &'a mut T;
}

/// Extension trait to add conversion to raw slices to slices.
pub trait SliceRawExt<T> {
    /// Converts the slice into a raw slice.
    fn as_raw(&self) -> *const [T];

    /// Converts the mutable slice into a mutable raw slice.
    fn as_mut_raw(&mut self) -> *mut [T];
}


impl<T> SliceRawExt<T> for [T] {
    fn as_raw(&self) -> *const [T] {
        self as *const [T]
    }

    fn as_mut_raw(&mut self) -> *mut [T] {
        self as *mut [T]
    }
}



impl<T> RawSlice<T> for *const [T] {
    unsafe fn as_slice<'a>(self) -> &'a [T] {
        &*self
    }

    unsafe fn slice(self, from: usize, to: usize) -> *const [T] {
        self.as_ptr().add(from).as_raw_slice(to - from)
    }
}

impl<T> RawSlice<T> for *mut [T] {
    unsafe fn as_slice<'a>(self) -> &'a [T] {
        &*self
    }

    unsafe fn slice(self, from: usize, to: usize) -> *mut [T] {
        self.as_mut_ptr().add(from).as_raw_mut_slice(to - from)
    }
}

impl<T> RawMutSlice<T> for *mut [T] {
    unsafe fn as_mut_slice<'a>(self) -> &'a mut[T] {
        &mut *self
    }

    fn as_mut_ptr(self) -> *mut T {
        unsafe { self.as_mut_slice().as_mut_ptr() }
    }

    unsafe fn write(self, index: usize, val: T) {
        self.as_mut_ptr().add(index).write(val);
    }

    unsafe fn write_bytes(self, byte: u8) {
        let len = self.len();
        self.as_mut_ptr().write_bytes(byte, len);
    }

    unsafe fn copy(self, from: *const[T]) {
        from.as_ptr().copy(self.as_mut_ptr(), from.len());
    }

    unsafe fn copy_nonoverlapping(self, from: *const[T]) {
        from.as_ptr().copy_nonoverlapping(self.as_mut_ptr(), from.len());
    }

    unsafe fn get_mut<'a>(self, index: usize) -> &'a mut T {
        &mut *self.as_mut_ptr().add(index)
    }
}
