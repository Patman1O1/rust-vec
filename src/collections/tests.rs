#[cfg(test)]

mod tests {
    mod functions {
        mod new {
            #[test]
            fn is_non_null() {
                let vec = crate::collections::vec::Vec::<i32>::new();
                assert!(!vec.as_ptr().is_null());
            }
        }
    }
}

