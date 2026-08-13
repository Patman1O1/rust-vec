use core::{
    mem,
    marker::PhantomData,
    hint::cold_path,
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
};

use std::{
    alloc::{self, Layout},
};

pub struct Vec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

#[inline(always)]
const fn is_zst<T>() -> bool { mem::size_of::<T>() == 0 }

#[inline(always)]
const fn dangling<T>() -> NonNull<T> {
    unsafe { NonNull::new_unchecked(mem::align_of::<T>() as *mut T)}
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Self {
            ptr: dangling(),
            len: 0,
            cap: if !is_zst::<T>() {
                0
            } else {
                cold_path();
                usize::MAX
            },
            _marker: PhantomData
        }
    }

    fn grow_to(new_len: usize) -> usize {
        
    }
} 