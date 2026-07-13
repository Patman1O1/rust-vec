pub mod collections {
    pub struct Vec<T> {
        ptr: std::ptr::NonNull<T>,
        len: usize,
        cap: usize
    }

    impl<T> Vec<T> {
        pub fn new() -> Self {
            if (std::mem::size_of::<T>() == 0) {
                
            }
            Vec {
                ptr: std::ptr::NonNull::dangling(),
                len: 0,
                cap: 0
            }
        }
    }
}