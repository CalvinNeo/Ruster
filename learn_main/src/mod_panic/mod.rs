use std::panic;

#[no_mangle]
pub extern "C" fn hello_from_rust_impl() {
    println!("Hello from Rust!");
    panic!("hello_from_rust_impl panicked");
}

fn test_panic_from_other_thread() {
    panic::set_hook(Box::new(move |info: &panic::PanicInfo<'_>| {
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };
        println!("Panic message is {} can_unwind {}", msg, info.can_unwind());
        let pl = info.payload();
        let pl = unsafe {
            Box::from_raw(pl as *const _ as *mut _)
        };
        panic::resume_unwind(pl);
    }));
    let t = std::thread::spawn(move || {
        hello_from_rust_impl();
    });
    std::thread::sleep(std::time::Duration::from_secs(1));
    let _ = t.join();
}
