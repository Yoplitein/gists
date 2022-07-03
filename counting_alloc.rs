use std::sync::atomic::{AtomicUsize, Ordering};
use std::alloc;

struct CountingAlloc {
    used: AtomicUsize,
}

impl CountingAlloc {
    pub const fn new() -> Self {
        Self {
            used: AtomicUsize::new(0)
        }
    }

    pub fn bytes_used(&self) -> usize {
        self.used.load(Ordering::SeqCst)
    }
}

unsafe impl alloc::GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        self.used.fetch_add(layout.size(), Ordering::SeqCst);
        alloc::System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::Layout) {
        self.used.fetch_sub(layout.size(), Ordering::SeqCst);
        alloc::System.dealloc(ptr, layout)
    }
}