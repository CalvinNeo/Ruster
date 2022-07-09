mod mod_path;

fn test_mod_path() {
    #[path = "mod_path/foo.rs"]
    mod c;
    // mod mod_path;
    println!("c {}", c::k());
    println!("b {}", mod_path::b::kb());
}

fn main() {
    test_mod_path();
}