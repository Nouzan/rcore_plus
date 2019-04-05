use core::mem;
use rcore_memory::PAGE_SIZE;
use log::*;
use crate::memory::{FRAME_ALLOCATOR, init_heap, MemoryAttr, MemorySet, Linear};
use crate::consts::{MEMORY_OFFSET, MEMORY_END, KERNEL_OFFSET};

/// Initialize the memory management module
pub fn init() {
    // initialize heap and Frame allocator
    init_frame_allocator();
    init_heap();
}

pub fn init_other() {
    // TODO: init other CPU cores
}

fn init_frame_allocator() {
    use bit_allocator::BitAlloc;
    use core::ops::Range;

    let mut ba = FRAME_ALLOCATOR.lock();
    let range = to_range((end as usize) - KERNEL_OFFSET + MEMORY_OFFSET + PAGE_SIZE, MEMORY_END);
    ba.insert(range);

    info!("frame allocator: init end");

    /// Transform memory area `[start, end)` to integer range for `FrameAllocator`
    fn to_range(start: usize, end: usize) -> Range<usize> {
        let page_start = (start - MEMORY_OFFSET) / PAGE_SIZE;
        let page_end = (end - MEMORY_OFFSET - 1) / PAGE_SIZE + 1;
        assert!(page_start < page_end, "illegal range for frame allocator");
        page_start..page_end
    }
}

// First core stores its SATP here.
// Other cores load it later.
static mut SATP: usize = 0;

pub unsafe fn clear_bss() {
    let start = sbss as usize;
    let end = ebss as usize;
    let step = core::mem::size_of::<usize>();
    for i in (start..end).step_by(step) {
        (i as *mut usize).write(0);
    }
}

// Symbols provided by linker script
#[allow(dead_code)]
extern {
    fn stext();
    fn etext();
    fn sdata();
    fn edata();
    fn srodata();
    fn erodata();
    fn sbss();
    fn ebss();
    fn start();
    fn end();
    fn bootstack();
    fn bootstacktop();
}