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
    result::{Result}
};

use std::{
    alloc::{
        self, System, handle_alloc_error, realloc,
    }, collections::TryReserveError
};

static INITIAL_CAPACITY: usize = 4;

pub struct Vec<T, A: GlobalAlloc = System> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
    alloc: A,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

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

    fn alloc(&mut self, size: usize) {
        let layout = Layout::array::<T>(size).expect("layout overflow");

        // SAFETY: `size` is guarenteed to be non-zero because self.alloc() 
        // is private and will only ever be called with a non-zero argument
        let raw_ptr: *mut u8 = unsafe {
            self.alloc.alloc(layout)
        };

        if raw_ptr.is_null() {
            cold_path();
            handle_alloc_error(layout);
        }

        // SAFETY: `raw_ptr` is non-null
        self.ptr = unsafe { NonNull::new_unchecked(raw_ptr as *mut T) };
        self.cap = size;
    }

    fn realloc(&mut self, new_size: usize) {
        let curr_layout = Layout::array::<T>(self.cap)
            .expect("layout overflow");
        
        // SAFETY: `new_size` is guarenteed to be non-zero because 
        // self.realloc() is private and will only ever be called with a
        // non-zero argument
        let raw_ptr: *mut u8 = unsafe {
            self.alloc.realloc(
                self.ptr.as_ptr() as *mut u8,
                curr_layout,
                new_size
            )
        };

        if raw_ptr.is_null() {
            handle_alloc_error(curr_layout);
        }

        // SAFETY: `raw_ptr` is non-null
        self.ptr = unsafe { NonNull::new_unchecked(raw_ptr as *mut T) };
        self.cap = new_size;
    }

    fn dealloc(&mut self) {
        unsafe {
            self.alloc.dealloc(
                self.ptr.as_ptr() as *mut u8,
                Layout::array::<T>(self.cap).expect("layout overflow")
            );
        };
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
} 

impl<T> Vec<T> {
    
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const T { self.ptr.as_ptr() }

    #[inline(always)]
    pub const fn capacity(&self) -> usize { self.cap }

    #[inline(always)]
    pub const fn len(&self) -> usize { self.len }
}

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

