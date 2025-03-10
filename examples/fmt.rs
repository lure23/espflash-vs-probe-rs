/*
* Adopted from -> TrouBLE examples/apps/src/fmt.rs
*/
#![macro_use]
#![allow(unused_macros)]

use core::fmt::{Debug, Display, LowerHex};

macro_rules! assert {
    ($($x:tt)*) => {
        {
            #[cfg(not(feature = "_defmt"))]
            ::core::assert!($($x)*);
            #[cfg(feature = "_defmt")]
            ::defmt::assert!($($x)*);
        }
    };
}

macro_rules! assert_eq {
    ($($x:tt)*) => {
        {
            #[cfg(not(feature = "_defmt"))]
            ::core::assert_eq!($($x)*);
            #[cfg(feature = "_defmt")]
            ::defmt::assert_eq!($($x)*);
        }
    };
}

macro_rules! assert_ne {
    ($($x:tt)*) => {
        {
            #[cfg(not(feature = "_defmt"))]
            ::core::assert_ne!($($x)*);
            #[cfg(feature = "_defmt")]
            ::defmt::assert_ne!($($x)*);
        }
    };
}

macro_rules! debug_assert {
    ($($x:tt)*) => {
        {
            #[cfg(not(feature = "_defmt"))]
            ::core::debug_assert!($($x)*);
            #[cfg(feature = "_defmt")]
            ::defmt::debug_assert!($($x)*);
        }
    };
}

macro_rules! debug_assert_eq {
    ($($x:tt)*) => {
        {
            #[cfg(not(feature = "_defmt"))]
            ::core::debug_assert_eq!($($x)*);
            #[cfg(feature = "_defmt")]
            ::defmt::debug_assert_eq!($($x)*);
        }
    };
}

macro_rules! debug_assert_ne {
    ($($x:tt)*) => {
        {
            #[cfg(not(feature = "_defmt"))]
            ::core::debug_assert_ne!($($x)*);
            #[cfg(feature = "_defmt")]
            ::defmt::debug_assert_ne!($($x)*);
        }
    };
}

macro_rules! todo {
    ($($x:tt)*) => {
        {
            #[cfg(not(feature = "_defmt"))]
            ::core::todo!($($x)*);
            #[cfg(feature = "_defmt")]
            ::defmt::todo!($($x)*);
        }
    };
}

#[cfg(not(feature = "_defmt"))]
macro_rules! unreachable {
    ($($x:tt)*) => {
        ::core::unreachable!($($x)*)
    };
}

#[cfg(feature = "_defmt")]
macro_rules! unreachable {
    ($($x:tt)*) => {
        ::defmt::unreachable!($($x)*)
    };
}

macro_rules! panic {
    ($($x:tt)*) => {
        {
            #[cfg(not(feature = "_defmt"))]
            ::core::panic!($($x)*);
            #[cfg(feature = "_defmt")]
            ::defmt::panic!($($x)*);
        }
    };
}

macro_rules! trace {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            cfg_if::cfg_if! {
                if #[cfg(feature = "_defmt")] {
                    ::defmt::trace!($s $(, $x)*);
                } else if #[cfg(feature = "_log")] {
                    ::log::trace!($s $(, $x)*);
                } else {
                    ::esp_println::println!($s $(, $x)*);
                }
            }
        }
    };
}

macro_rules! debug {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            cfg_if::cfg_if! {
                if #[cfg(feature = "_defmt")] {
                    ::defmt::debug!($s $(, $x)*);
                } else if #[cfg(feature = "_log")] {
                    ::log::debug!($s $(, $x)*);
                } else {
                    ::esp_println::println!($s $(, $x)*);
                }
            }
        }
    };
}

macro_rules! info {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            cfg_if::cfg_if! {
                if #[cfg(feature = "_defmt")] {
                    ::defmt::info!($s $(, $x)*);
                } else if #[cfg(feature = "_log")] {
                    ::log::info!($s $(, $x)*);
                } else {
                    ::esp_println::println!($s $(, $x)*);
                }
            }
        }
    };
}

macro_rules! warn {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            cfg_if::cfg_if! {
                if #[cfg(feature = "_defmt")] {
                    ::defmt::warn!($s $(, $x)*);
                } else if #[cfg(feature = "_log")] {
                    ::log::warn!($s $(, $x)*);
                } else {
                    ::esp_println::println!($s $(, $x)*);
                }
            }
        }
    };
}

macro_rules! error {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            cfg_if::cfg_if! {
                if #[cfg(feature = "_defmt")] {
                    ::defmt::error!($s $(, $x)*);
                } else if #[cfg(feature = "_log")] {
                    ::log::error!($s $(, $x)*);
                } else {
                    ::esp_println::println!($s $(, $x)*);
                }
            }
        }
    };
}

#[cfg(feature = "_defmt")]
macro_rules! unwrap {
    ($($x:tt)*) => {
        ::defmt::unwrap!($($x)*)
    };
}

#[cfg(not(feature = "_defmt"))]
macro_rules! unwrap {
    ($arg:expr) => {
        match $crate::fmt::Try::into_result($arg) {
            ::core::result::Result::Ok(t) => t,
            ::core::result::Result::Err(e) => {
                ::core::panic!("unwrap of `{}` failed: {:?}", ::core::stringify!($arg), e);
            }
        }
    };
    ($arg:expr, $($msg:expr),+ $(,)? ) => {
        match $crate::fmt::Try::into_result($arg) {
            ::core::result::Result::Ok(t) => t,
            ::core::result::Result::Err(e) => {
                ::core::panic!("unwrap of `{}` failed: {}: {:?}", ::core::stringify!($arg), ::core::format_args!($($msg,)*), e);
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NoneError;

pub trait Try {
    type Ok;
    type Error;
    #[allow(dead_code)]
    fn into_result(self) -> Result<Self::Ok, Self::Error>;
}

impl<T> Try for Option<T> {
    type Ok = T;
    type Error = NoneError;

    #[inline]
    fn into_result(self) -> Result<T, NoneError> {
        self.ok_or(NoneError)
    }
}

impl<T, E> Try for Result<T, E> {
    type Ok = T;
    type Error = E;

    #[inline]
    fn into_result(self) -> Self {
        self
    }
}

#[allow(unused)]
pub(crate) struct Bytes<'a>(pub &'a [u8]);

impl<'a> Debug for Bytes<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#02x?}", self.0)
    }
}

impl<'a> Display for Bytes<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#02x?}", self.0)
    }
}

impl<'a> LowerHex for Bytes<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#02x?}", self.0)
    }
}

#[cfg(feature = "_defmt")]
impl<'a> defmt::Format for Bytes<'a> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{:02x}", self.0)
    }
}
