#[macro_export]
macro_rules! python {
    ($($code:tt)*) => {
        run_python(stringify!($($code)*))}
}
