/// 理论上不可达用这个, 返回默认值
#[macro_export]
macro_rules! unreachable {
    () => ({
        dbg!("@@@@@@@@");
        $crate::log::error!("internal error: entered unreachable code");
        Default::default()
    });
    ($msg:expr) => ({
        $crate::unreachable!("{}", $msg)
    });
    ($msg:expr,) => ({
        $crate::unreachable!($msg)
    });
    ($fmt:expr, $($arg:tt)*) => ({
        $crate::log::error!(concat!("internal error: entered unreachable code: ", $fmt), $($arg)*);
        Default::default()
    });
}

/// 理论上不可达用这个, 返回指定的默认值
#[macro_export]
macro_rules! unreachable_with {
    ($e:expr) => ({
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
        let _t: i32 = unreachable!();
        assert_eq!(2 + 2, 4);
    }
}
