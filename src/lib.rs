#![no_std]

#![feature(const_fn)]

/// Provides an unsafe way to late-initialize static variables that will see a lot of use.
/// Basically a wrapper around UnsafeCell that only permits setting once.

use core::{
    ops::Deref,
    cell::UnsafeCell,
    clone::Clone,
    convert::AsRef,
    fmt::{
        Display,
        Debug,
        Formatter,
        Error as FmtError
    }
};

// We use UnsafeCell because we need interior mutability, and we're not using Cell because we don't
//  want any runtime cost. There isn't any principled reason this is UnsafeCell<Option> rather than
//  Option<UnsafeCell>, so if performance is better one way or the other this may change.
pub struct LateInit<T>(UnsafeCell<Option<T>>);

unsafe impl <T> Sync for LateInit<T> {}

impl <T> LateInit<T> {
    pub const fn new() -> Self {
        LateInit(UnsafeCell::new(None))
    }

    pub unsafe fn init(&self, value: T) {
        #[cfg(not(feature = "unchecked"))] {
            assert!((*self.0.get()).is_none(), "LateInit.init called more than once");
        }

        *self.0.get() = Some(value);
    }

    #[inline(always)]
    fn option(&self) -> &Option<T> {
        unsafe { &*self.0.get() }
    }

    #[inline(always)]
    fn data(&self) -> &T {
        match self.option() {
            Some(ref x) => x,
            _ => unreachable!(),
        }
    }
}

impl <T: Clone> LateInit<T> {
    #[inline(always)]
    pub fn clone(&self) -> T {
        self.data().clone()
    }
}

impl <T> Deref for LateInit<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        #[cfg(not(feature = "unchecked"))] {
            debug_assert!(self.option().is_some(), "LateInit used without initialization");
        }
        self.data()
    }
}

impl <W, T: AsRef<W>> AsRef<W> for LateInit<T> {
    #[inline(always)]
    fn as_ref(&self) -> &W {
        #[cfg(not(feature = "unchecked"))] {
            debug_assert!(self.option().is_some(), "LateInit used without initialization");
        }
        self.data().as_ref()
    }
}

impl <T: Debug> Debug for LateInit<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self.option() {
            Some(ref x) => { x.fmt(f) },
            None => { write!(f, "<UNINITIALIZED>") },
        }
    }
}

impl <T: Display> Display for LateInit<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self.option() {
            Some(ref x) => { x.fmt(f) },
            None => { write!(f, "<UNINITIALIZED>") },
        }
    }
}