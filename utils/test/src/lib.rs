#[doc(hidden)]
pub use difference::Changeset;

#[macro_export]
macro_rules! assert_diff {
    ($left:expr, $right:expr) => {
        assert_diff!($left, $right,)
    };
    ($left:expr, $right:expr, $($tt:tt)*) => {{
        let left = $left;
        let right = $right;
        if left != right {
            if left.trim() == right.trim() {
                eprintln!("Left:\n{:?}\n\nRight:\n{:?}\n\nWhitespace difference\n", left, right);
            } else {
                let changeset = $crate::Changeset::new(right.as_ref(), left.as_ref(), "\n");
                eprintln!("Left:\n{}\n\nRight:\n{}\n\nDiff:\n{}\n", left, right, changeset);
            }
            eprintln!($($tt)*);
            panic!("'assertion failed: `(left == right)`");
        }
    }};
}
