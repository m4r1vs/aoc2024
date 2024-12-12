#[macro_export()]
macro_rules! wradd {
    ($val:expr, $($offset:expr),+) => {
        $val $(.wrapping_add_signed($offset))+
    };
}
