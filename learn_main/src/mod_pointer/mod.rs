use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};
use std::mem::{size_of, size_of_val};

fn test_cell() {
    let ic = Cell::new(1);
    // Can't compile
    // (*ic.get_mut()) = 3;
    // *ic = 3;
    let irc = RefCell::new(1);
    (*irc.borrow_mut()) = 3;
    assert_eq!((*irc.borrow()), 3);
}

fn test_size() {
    assert_eq!(size_of::<u32>(), 4);
    assert_eq!(size_of::<usize>(), 8);
    assert_eq!(size_of::<[u32; 2]>(), 8);
    assert_eq!(size_of::<&u32>(), 8);
    assert_eq!(size_of::<&[u32; 2]>(), 8);
    assert_eq!(size_of::<&[u32]>(), 16);
    assert_eq!(size_of::<&mut [u32]>(), 16);
    assert_eq!(size_of::<*mut [u32]>(), 16);
    assert_eq!(size_of::<*const [u32]>(), 16);

    let arr = [u64::MAX; 4];
    let parr: *const [u64; 4] = &arr;
    let ps: *const [u64] = &arr;
    assert_eq!(size_of_val(&arr), 8 * 4);
    assert_eq!(size_of_val(&parr), 8);
    assert_eq!(size_of_val(&ps), 16);
    unsafe {
        assert_eq!(size_of_val(&*parr), 8 * 4);
        assert_eq!(size_of_val(&*ps), 8 * 4);
    }
    unsafe {
        let ps8 = ps as *const [u8];
        assert_eq!(size_of_val(&ps8), 16);
        // If affects only the pointer to data.
        // Pointer to length remains intact.
        assert_eq!(size_of_val(&*ps8), 1 * 4);
    }
}

pub fn run() {
    test_cell();
    test_size();
}
