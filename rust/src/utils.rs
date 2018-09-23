use std::process;

pub trait UnwrapAbort {
    type Target;
    fn unwrap_abort(self) -> Self::Target;
}

impl<T> UnwrapAbort for Option<T> {
    type Target = T;

    #[inline]
    fn unwrap_abort(self) -> T {
        match self {
            Some(v) => v,
            None => process::abort(),
        }
    }
}

impl<T, E> UnwrapAbort for Result<T, E> {
    type Target = T;

    #[inline]
    fn unwrap_abort(self) -> T {
        match self {
            Ok(v) => v,
            Err(_) => process::abort(),
        }
    }
}
