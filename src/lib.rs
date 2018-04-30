#![no_std]

#![feature(const_fn)]
#![feature(optin_builtin_traits)]

//! Provides an unsafe way to late-initialize static variables that will see a lot of use.
//! Essentially a wrapper around `UnsafeCell` that only permits setting the contained value once.
//!
//! Usage:
//!
//! ```
//! # use lateinit::LateInit;
//! static SOMETHING: LateInit<String> = LateInit::new();
//!
//! unsafe { SOMETHING.init("hello world".to_owned()); }
//! println!("{}", SOMETHING);
//! ```
//!
//! Multiple-initialization causes a panic:
//! ```should_panic
//! # use lateinit::LateInit;
//! static SOMETHING: LateInit<String> = LateInit::new();
//!
//! unsafe {
//!     SOMETHING.init("something".to_owned());
//!     SOMETHING.init("something else".to_owned());
//! }
//! ```

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

/// The primary type for this crate. Initialize before use.
// We use UnsafeCell because we need interior mutability, and we're not using Cell because we don't
//  want any runtime cost. There isn't any principled reason this is UnsafeCell<Option> rather than
//  Option<UnsafeCell>, so if performance is better one way or the other this may change.
pub struct LateInit<T>(UnsafeCell<Option<T>>);

// Sync is allowed
unsafe impl <T> Sync for LateInit<T> {}
impl <T> !Send for LateInit<T> {}

impl <T> LateInit<T> {
    /// Create a new LateInit.
    pub const fn new() -> Self {
        LateInit(UnsafeCell::new(None))
    }

    /// Assign a value. Panics if called more than once.
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
    /// Clone contained value. Panics in debug profile if called before initialization.
    ///
    /// Note that `Clone` is not implemented because `LateInit` doesn't
    /// support mutation, so `clone_from` is impossible.
    #[inline(always)]
    pub fn clone(&self) -> T {
        #[cfg(not(feature = "unchecked"))] {
            debug_assert!(self.option().is_some(), "LateInit used without initialization");
        }

        self.data().clone()
    }
}

impl <T> Deref for LateInit<T> {
    type Target = T;

    /// Deref to contained value. Panics in debug if called before initialization.
    #[inline(always)]
    fn deref(&self) -> &T {
        #[cfg(not(feature = "unchecked"))] {
            debug_assert!(self.option().is_some(), "LateInit used without initialization");
        }
        self.data()
    }
}

impl <T> AsRef<T> for LateInit<T> {
    /// Panics in debug if called before initialization.
    #[inline(always)]
    fn as_ref(&self) -> &T {
        #[cfg(not(feature = "unchecked"))] {
            debug_assert!(self.option().is_some(), "LateInit used without initialization");
        }
        self.data()
    }
}

impl <T: Debug> Debug for LateInit<T> {
    /// Delegates to `Debug` implementation on contained value. This is a checked access.
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self.option() {
            Some(ref x) => { x.fmt(f) },
            None => { write!(f, "<UNINITIALIZED>") },
        }
    }
}

impl <T: Display> Display for LateInit<T> {
    /// Delegates to `Display` implementation on contained value. This is a checked access.
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self.option() {
            Some(ref x) => { x.fmt(f) },
            None => { write!(f, "<UNINITIALIZED>") },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use core::convert::AsRef;
    use core::ops::Deref;

    #[test]
    #[should_panic]
    fn multiple_init_panics() {
        let li = LateInit::<usize>::new();
        unsafe {
            li.init(4);
            li.init(4);
        }
    }

    #[test]
    #[should_panic]
    fn as_ref_panics() {
        let li = LateInit::<usize>::new();
        let _ = li.as_ref();
    }

    #[test]
    #[should_panic]
    fn deref_panics() {
        let li = LateInit::<usize>::new();
        let _ = li.deref();
    }
}