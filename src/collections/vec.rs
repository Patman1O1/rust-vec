use core::{
    alloc::{
        GlobalAlloc,
        Layout
    },
    mem::{
        size_of,
        align_of
    },
    marker::PhantomData,
    hint::cold_path,
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
};

use std::{
    alloc::{self, System, handle_alloc_error}
};

pub struct Vec<T, A: GlobalAlloc = System> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
    alloc: A,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

#[inline(always)]
const fn is_zst<T>() -> bool { size_of::<T>() == 0 }

#[inline(always)]
const fn dangling<T>() -> NonNull<T> {
    unsafe { NonNull::new_unchecked(align_of::<T>() as *mut T)}
}

#[inline(always)]
const fn default_cap<T>() -> usize {
    if !is_zst::<T>() { 0 } else { usize::MAX }
}

impl<T, A: GlobalAlloc> Vec<T, A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            ptr: dangling(),
            len: 0,
            cap: if !is_zst::<T>() {
                0
            } else {
                cold_path();
                usize::MAX
            },
            alloc,
            _marker: PhantomData
        }
    }
}

impl<T> Vec<T, System> {
    pub const fn new() -> Self {
        Self {
            ptr: dangling(),
            len: 0,
            cap: default_cap::<T>(),
            alloc: System,
            _marker: PhantomData
        }
    }

    pub fn with_capacity(capacity: usize) -> Vec<T, System> {
        let mut vec: Vec<T, System> = Self {
            ptr: dangling(),
            len: 0,
            cap: capacity,
            alloc: System,
            _marker: PhantomData
        };

        vec.realloc(capacity);

        vec
    }



    fn realloc(&mut self, new_len: usize) {
        let new_layout = Layout::array::<T>(new_len).expect("layout overflow");

        // Allocate a new contiguous block of memory on the heap
        let new_ptr: *mut u8 = if self.len > 0 {
            let curr_layout = Layout::array::<T>(self.len).expect("layout overflow");
            unsafe { alloc::realloc(self.ptr.as_ptr() as *mut u8, curr_layout, new_len) }
        } else {
            cold_path();
            unsafe { alloc::alloc(new_layout) }
        };

        // Check if the new pointer is non-null
        if new_ptr.is_null() {
            cold_path();
            handle_alloc_error(new_layout);
        }

        self.ptr = unsafe { NonNull::new_unchecked(new_ptr as *mut T) };
        self.cap = new_layout.size();

    }

} 

impl<T> Vec<T> {
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const T { self.ptr.as_ptr() }

    #[inline(always)]
    pub const fn capacity(&self) -> usize { self.cap }

    #[inline(always)]
    pub const fn len(&self) -> usize { self.len }
}
