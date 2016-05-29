/// Utility Functions to make my life easier

/// This acts like the try!() macro but instead panics on failure rather
/// than returning an error for the result allowing a function to not
/// require Result as a return type
#[macro_export]
macro_rules! try_panic {
    ($x:expr) => {
        match $x {
            Ok(y) => y,
            Err(e) => panic!(e),
        };
    };
}

/// Used to unwrap Option types inside Results that panics on Err or None
#[macro_export]
macro_rules! shuck_panic {
    ($x:expr) => {
        match $x {
            Ok(y) => {
                match y {
                    Some(x) => x,
                    None    => panic!("None detected. Expected a Some value"),
                }
            },
            Err(e) => panic!(e),
        };
    };
}

/// Used to unwrap Option types inside Results that does not panic on None
macro_rules! shuck {
    ($x:expr) => {
        match $x {
            Ok(y) => {
                match y {
                    Some(x) => x,
                    None    => None,
                }
            },
            Err(e) => panic!(e),
        };
    };
}
