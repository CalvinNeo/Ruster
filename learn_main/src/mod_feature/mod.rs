struct DebugStruct {
    pub a: String,
}

#[cfg(test)]
impl DebugStruct {
    fn new() -> Self {
        DebugStruct { a: format!("test") }
    }
}

#[cfg(not(test))]
impl DebugStruct {
    fn new() -> Self {
        DebugStruct {
            a: format!("normal"),
        }
    }
}

struct S {
    pub b: i32,
    pub d: DebugStruct,
}

pub fn run() {
    let mut s = S {
        b: 1,
        d: DebugStruct::new(),
    };
    assert_eq!(s.d.a, format!("normal"));
}

#[cfg(test)]
mod tests {
    use crate::mod_feature::*;

    #[test]
    fn test_s() {
        let mut s = S {
            b: 1,
            d: DebugStruct::new(),
        };
        assert_eq!(s.d.a, format!("test"));
    }
}
