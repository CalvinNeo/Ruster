use std::vec::Vec;

pub fn k() {
    println!("k {}", m!(1));
    println!("k2 {}", m2!(1));
}

// macro_rules! v {
//     ($($x:expr),+ $(,)?) => (
//         $x
//     );
// }

macro_rules! add_list {
    ($($a:expr),*) => {
        0
        $(+$a)*
    }
}

macro_rules! add_list2 {
    ($a:expr) => {
        $a
    };
    ($a:expr,$($b:expr),+) => {
        $a
        $(+$b)*
    };
}

macro_rules! add_list3 {
    ($a:expr) => {
        $a
    };
    ($a:expr,$($tail:tt)*) => {
        $a+add_list3!($($tail)*)
    };
}

pub fn k2() {
    assert_eq!(add_list!(1, 2, 3), 6);
    assert_eq!(add_list2!(1, 2, 3), 6);
    assert_eq!(add_list3!(1, 2, 3), 6);
}
