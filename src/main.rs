

mod mod_path;
mod mod_macro;
mod mod_feature;

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

fn main() {
    test_mod_path();
    test_mod_macro();
    test_mod_feature();
}