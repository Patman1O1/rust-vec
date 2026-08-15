#[cfg(test)]
mod tests {
    // Core Aliases
    use core::marker::PhantomData;

    // Local Aliases
    use crate::collections::vec::Vec;

    mod functions {
        // Parent Aliases
        use super::*;

        mod new {
            // Parent Aliases
            use super::*;

            #[test]
            fn is_non_null() {
                let vec = Vec::<i32>::new();
                assert!(!vec.as_ptr().is_null());
            }
            
            #[test]
            fn cap_zero_size_type() {
                let vec = Vec::<PhantomData<i32>>::new();
                assert_eq!(usize::MAX, vec.capacity());
            }
            
            #[test]
            fn cap_non_zero_size_type() {
                let vec = Vec::<i32>::new();
                assert_eq!(0, vec.capacity());
            }
        }

        mod new_in {
            // Core Aliases
            use core::marker::PhantomData;

            // Standard Library Aliases
            use std::alloc::System;

            // Local Aliases
            use crate::collections::vec::Vec as MyVec;

            #[test]
            fn is_non_null() {
                let vec = MyVec::<i32, System>::new_in(System);
                assert!(!vec.as_ptr().is_null());
            }

            #[test]
            fn cap_zero_size_type() {
                let vec = MyVec::<PhantomData<i32>, System>::new_in(System);
                assert_eq!(usize::MAX, vec.capacity());
            }

            #[test]
            fn cap_non_zero_size_type() {
                let vec = MyVec::<i32, System>::new_in(System);
                assert_eq!(0, vec.capacity());
            }
        }
    }
}

