/// Helper trait to abstract over `Option` vs. `Result`.
pub trait Success<T> {
    fn success(self) -> Option<T>;
}

impl<T> Success<T> for Option<T> {
    fn success(self) -> Option<T> {
        self
    }
}

impl<T, E> Success<T> for Result<T, E> {
    fn success(self) -> Option<T> {
        self.ok()
    }
}

/// Warn about a failed unwrap.
#[macro_export]
macro_rules! warn_unwrap {
    ($expr:expr) => {
        warn!(
            "Unwrap failed at {}:{}:{}: `{}`",
            file!(),
            line!(),
            column!(),
            stringify!($expr),
        );
    };
}

/// Unwrap or return with a warning.
#[macro_export]
macro_rules! r {
    ($return:expr, $expr:expr $(,)?) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => {
                $crate::warn_unwrap!($expr);
                return $return;
            },
        }
    };

    ($expr:expr $(,)?) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => {
                $crate::warn_unwrap!($expr);
                return Default::default();
            },
        }
    };
}

/// Unwrap or return quietly.
#[macro_export]
macro_rules! rq {
    ($return:expr, $expr:expr $(,)?) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => return $return,
        }
    };

    ($expr:expr $(,)?) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => return Default::default(),
        }
    };
}

/// Unwrap or continue with a warning.
#[macro_export]
macro_rules! c {
    ($expr:expr) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => {
                $crate::warn_unwrap!($expr);
                continue;
            },
        }
    };
}

/// Unwrap or continue quietly.
#[macro_export]
macro_rules! cq {
    ($expr:expr) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => continue,
        }
    };
}
