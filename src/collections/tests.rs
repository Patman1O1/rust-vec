#[cfg(test)]

mod tests {
    mod functions {
        mod new {
            // Core Aliases
            use core::marker::PhantomData;
            
            // Local Aliases
            use crate::collections::vec::Vec as MyVec;

            #[test]
            fn is_non_null() {
                let vec = MyVec::<i32>::new();
                assert!(!vec.as_ptr().is_null());
            }
            
            #[test]
            fn cap_zero_size_type() {
                let vec = MyVec::<PhantomData<i32>>::new();
                assert_eq!(usize::MAX, vec.capacity());
            }
            
            #[test]
            fn cap_non_zero_size_type() {
                let vec = MyVec::<i32>::new();
                assert_eq!(0, vec.capacity());
            }
        }

        
    }
}

