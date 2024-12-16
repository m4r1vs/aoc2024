#[macro_export()]
macro_rules! wradd {
    ($val:expr, $($offset:expr),+) => {
        $val $(.wrapping_add_signed($offset))+
    };
}

#[macro_export()]
macro_rules! diradd {
    ($pos:expr, $delta:expr) => {
        (wradd!($pos.0, $delta.0), wradd!($pos.1, $delta.1))
    };
}

#[macro_export()]
macro_rules! dirsub {
    ($pos:expr, $delta:expr) => {
        (wradd!($pos.0, -$delta.0), wradd!($pos.1, -$delta.1))
    };
}
