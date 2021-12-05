use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

struct Counter;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), SeqCst);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), SeqCst);
    }
}

#[global_allocator]
static A: Counter = Counter;

pub fn count_allocations() -> usize {
    ALLOCATED.load(SeqCst)
}

pub struct AllocatorCounter {
    start_value: usize,
}
impl AllocatorCounter {
    pub fn reset(&mut self) -> usize {
        let start = self.start_value;
        self.start_value = count_allocations();
        start - self.start_value
    }

    pub fn count_and_reset(&mut self) -> usize {
        self.reset()
    }

    pub fn count(&self) -> usize {
        count_allocations() - self.start_value
    }

    pub fn measure<F, R>(&self, func: F) -> MeasureResult<R>
    where
        F: FnOnce() -> R,
    {
        let start = count_allocations();
        let result = func();
        let allocations = count_allocations() - start;
        MeasureResult {
            result,
            allocations,
        }
    }
}

pub struct MeasureResult<R> {
    pub result: R,
    pub allocations: usize,
}

impl Default for AllocatorCounter {
    fn default() -> Self {
        AllocatorCounter {
            start_value: count_allocations(),
        }
    }
}
