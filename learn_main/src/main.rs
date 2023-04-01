#![feature(negative_impls)]
#![feature(panic_can_unwind)]

mod mod_feature;
mod mod_lifetime;
mod mod_macro;
mod mod_path;
mod mod_pin;
mod mod_pointer;
mod mod_test_macro;
mod mod_thread;
mod mod_panic;

fn test_mod_path() {
    #[path = "mod_path/foo.rs"]
    mod c;
    // mod mod_path;
    println!("c {}", c::k());
    println!("b {}", mod_path::b::kb());
}

fn test_mod_macro() {
    mod_macro::a::k();
    use crate::helped;
    fn unit() {
        helped!();
    }
    mod_macro::a::k2();
}

fn test_mod_feature() {
    mod_feature::run();
}

fn test_mod_pin() {
    mod_pin::run();
}

fn test_mod_pointer() {
    mod_pointer::run();
}

fn test_mod_lifetime() {
    mod_lifetime::run();
}

fn main() {
    test_mod_path();
    test_mod_macro();
    test_mod_feature();
    test_mod_pin();
    test_mod_pointer();
    mod_thread::run();
}
