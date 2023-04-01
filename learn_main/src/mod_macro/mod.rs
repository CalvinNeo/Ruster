macro_rules! m {
    (1) => {
        2
    };
}

#[macro_use]
pub mod inner {
    macro_rules! m2 {
        (1) => {
            3
        };
    }
}

pub mod a;

#[macro_export]
macro_rules! helped {
    // () => { helper!() } // This might lead to an error due to 'helper' not being in scope.
    () => {
        $crate::helper!()
    };
}

#[macro_export]
macro_rules! helper {
    () => {
        ()
    };
}
