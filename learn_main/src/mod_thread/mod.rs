pub fn test_join_panic() {
    let thread_join_handle = std::thread::spawn(move || {
        panic!("Pan");
    });
    let res = thread_join_handle.join();
    assert!(res.is_err());
}

pub fn run() {
    test_join_panic();
}
