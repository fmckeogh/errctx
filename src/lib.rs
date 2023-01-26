#![deny(missing_docs)]

//! Wrapper for tacking on path context

use std::{
    error::Error,
    fmt::{self, Debug, Display},
    path::{Path, PathBuf},
};

/// Wrapper around an error providing additional context.
#[derive(Debug)]
pub struct ErrCtx<E, T> {
    /// Inner error
    pub inner: E,
    /// Context associated with error
    pub ctx: T,
}

impl<E: Error + 'static, T: Debug> Error for ErrCtx<E, T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use thiserror::__private::AsDynError;
        Some(self.inner.as_dyn_error())
    }
}

impl<E, T> ErrCtx<E, T> {
    /// Creates a new `ErrCtx` from an error and context type.
    pub fn new(error: E, ctx: T) -> Self {
        Self { inner: error, ctx }
    }
}

impl<E, T: Debug> Display for ErrCtx<E, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.ctx)
    }
}

/// Additional path context for errors.
pub type PathCtx<E> = ErrCtx<E, PathBuf>;

impl<E> PathCtx<E> {
    /// Convenience method that creates a closure which converts an `io::Error` to a `IoErrCtx`
    ///
    /// This is designed for simplifying the conversion of `io::Error`s using `.map_err`.
    ///
    /// For example
    ///
    /// ```
    /// # use {std::{io, path::PathBuf}, errctx::PathCtx};
    /// #
    /// #   let res: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::Other, "oh no!"));
    /// #   let path = PathBuf::from("example");
    /// let res = res.map_err(|e| PathCtx::new(e, path));
    /// #   assert_eq!(format!("{res:?}"), "Err(ErrCtx { inner: Custom { kind: Other, error: \"oh no!\" }, ctx: \"example\" })");
    /// ```
    ///
    /// can become
    ///
    /// ```
    /// # use {std::{io, path::PathBuf}, errctx::PathCtx};
    /// #
    /// #   let res: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::Other, "oh no!"));
    /// #   let path = PathBuf::from("example");
    /// let res = res.map_err(PathCtx::f(path));
    /// #   assert_eq!(format!("{res:?}"), "Err(ErrCtx { inner: Custom { kind: Other, error: \"oh no!\" }, ctx: \"example\" })");
    /// ```
    pub fn f<P: AsRef<Path>>(path: P) -> Box<dyn FnOnce(E) -> Self> {
        let p = path.as_ref().to_owned();
        Box::new(|e| Self::new(e, p))
    }
}
