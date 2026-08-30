// Core Aliases
use core::{
   alloc::{
       GlobalAlloc
   },
   ptr::{
       NonNull
   },
   slice
};

// Parent Aliases
use super::Vec;

// Standard Library Aliases
use std::{
    alloc::{
        System
    }
};

pub struct Drain<'a, T: 'a, A: GlobalAlloc + 'a = System> {
    pub(super) tail_start: usize,
    pub(super) tail_len: usize,
    pub(super) iter: slice::Iter<'a, T>,
    pub(super) vec: NonNull<Vec<T, A>>
}

