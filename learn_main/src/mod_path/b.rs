#[path = "foo.rs"]
mod c;

pub fn kb() -> String {
    format!("kb: {}", c::k())
}
