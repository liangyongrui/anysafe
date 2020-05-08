/// 理论上不可达用这个, 返回默认值
#[macro_export]
macro_rules! unreachable {
    () => (
        $crate::unreachable_with!(Default::default())
    );
    ($msg:expr) => (
        $crate::unreachable_with!(Default::default(), $msg)
    );
    ($msg:expr,) => (
        $crate::unreachable_with!(Default::default(), $msg)
    );
    ($fmt:expr, $($arg:tt)*) => (
        $crate::unreachable_with!(Default::default(), $fmt, $($arg)*)
    );
}

/// 理论上不可达用这个, 返回指定的默认值
#[macro_export]
macro_rules! unreachable_with {
    ($e:expr) => ({
        $crate::log::error!("internal error: entered unreachable code");
        $e
    });
    ($e:expr,) => ({
        $crate::log::error!("internal error: entered unreachable code");
        $e
    });
    ($e:expr, $msg:expr) => ({
        $crate::unreachable_with!($e, "{}", $msg)
    });
    ($e:expr, $msg:expr,) => ({
        $crate::unreachable_with!($e, $msg)
    });
    ($e:expr, $fmt:expr, $($arg:tt)*) => ({
        $crate::log::error!(concat!("internal error: entered unreachable code: ", $fmt), $($arg)*);
        $e
    });
}
#[cfg(test)]
mod tests {
    use crate::unreachable;
    #[test]
    fn it_works() {
        let a: i32 = unreachable!();
        assert_eq!(a, 0);
        let b: i32 = unreachable!("1");
        assert_eq!(b, 0);
        let c: i32 = unreachable!("1",);
        assert_eq!(c, 0);
        let d: i32 = unreachable!("{}{}", 1, 3);
        assert_eq!(d, 0);
        let e: i32 = unreachable_with!(10);
        assert_eq!(e, 10);
        let f: i32 = unreachable_with!(10,);
        assert_eq!(f, 10);
        let g: i32 = unreachable_with!(10, "123");
        assert_eq!(g, 10);
        let h: i32 = unreachable_with!(10, "123",);
        assert_eq!(h, 10);
        let i: i32 = unreachable_with!(10, "{}", 1);
        assert_eq!(i, 10);
        let j: i32 = unreachable_with!(10, "{}", 1,);
        assert_eq!(j, 10);
    }
}
