mod mod_path;
mod mod_macro;

fn test_mod_path() {
    #[path = "mod_path/foo.rs"]
    mod c;
    // mod mod_path;
    println!("c {}", c::k());
    println!("b {}", mod_path::b::kb());
}

fn test_mod_macro() {
    mod_macro::a::k();
}

fn main() {
    test_mod_path();
    test_mod_macro();
}