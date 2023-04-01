#[link(name = "v1", kind = "dylib")]
extern {
    fn c_func();
}

fn main() {
    unsafe {
        c_func();
    }
}
