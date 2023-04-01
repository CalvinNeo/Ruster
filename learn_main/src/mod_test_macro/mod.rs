#![feature(custom_test_frameworks)]
use test_macro::test_case;
fn KKKKKKKKK() {}

mod AAAAAAAA {
    pub fn aaaaaaaa() {
        println!("aaaaaaaa");
    }
}

mod BBBBBBBB {
    pub fn bbbbbbbb() {
        println!("bbbbbbbb");
    }
}

#[test_case(AAAAAAAA::aaaaaaaa)]
#[allow(clippy::absurd_extreme_comparisons)]
#[test_case(BBBBBBBB::bbbbbbbb)]
fn TTTTTTTTTT() {
    KKKKKKKKK();
    fffffff();
}
