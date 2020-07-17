#![no_std]

//! Marker types to indicate precisely the [variance] relationship between a
//! generic type and its parameters.
//!
//! Rust supports three different modes of variance between a generic type `F`
//! and a type parameter `T`:<sup>[1]</sup>
//!  - Covariance: `F<T>` is a subtype of `F<U>` if `T` is a subtype of `U`.
//!  - Contravariance: `F<T>` is a subtype of `F<U>` if `U` is a subtype of `T`.
//!  - Invariance: `F<T>` is never a subtype of `F<U>` (unless `T = U`).
//!
//! Rust is usually able to infer the variance of a type parameter from its use,
//! but fails if the type parameter is not used within the type definition.
//! Typically, this is resolved by using a [`PhantomData`] to indicate the
//! parameter's use within the type:
//! ```
//! use std::marker::PhantomData;
//!
//! struct Slice<'a, T: 'a> {
//!     start: *const T,
//!     end: *const T,
//!     phantom: PhantomData<&'a T>,
//! }
//! ```
//!
//! However, in some cases, the subtyping relation is not always obvious from a
//! `PhantomData` field. In such cases, it can be useful to make the variance
//! explicit with one of the markers [`Covariant`], [`Contravariant`], and
//! [`Invariant`].
//! ```
//! use variance::{Covariant, Contravariant};
//!
//! struct Func<Arg, Ret> {
//!     arg: Covariant<Arg>,
//!     ret: Contravariant<Ret>,
//! }
//! ```
//!
//! ## Enforcing invariance
//!
//! Another use case is when a type parameter is used, but the Rust compiler
//! deduces a more permissive variance than is desired. In this case, the
//! `Invariant` marker can be used to ensure that the generic type is invariant
//! with respect to the given type parameter.
//! ```
//! use variance::Invariant;
//!
//! struct Opaque<T> {
//!     inner: Box<T>,        // Implies `Opaque` is covariant to `T`
//!     marker: Invariant<T>, // Ensures that `Opaque` is invariant to `T`
//! }
//! ```
//! The `Invariant` overrules any other implied variances and so `Opaque`
//! becomes invariant to `T`.
//!
//! # Limitations
//!
//! The marker traits `Covariant` and `Contravariant` _do not_ necessarily
//! guarantee that the compiler will use the marked variance. If two uses of a
//! type parameter imply differing variances, the compiler will consider the
//! generic type _invariant_ with respect to the parameter.
//!
//! For example:
//! ```
//! # use variance::Contravariant;
//! #
//! struct Ref<'a, T> {
//!     inner: &'a T,             // Implies `Ref` is covariant to `T`
//!     marker: Contravariant<T>, // Implies `Ref` is contravariant to `T`
//! }
//! ```
//! As a result of these conflicting variances, the compiler will decide that
//! `Ref` is invariant to `T`.
//!
//! Due to this, it is recommended that `Covariant` and `Contravariant` are only
//! used on type parameters that are not used in any other fields of the type.
//!
//! [variance]: https://en.wikipedia.org/wiki/Covariance_and_contravariance_(computer_science)
//! [1]: https://doc.rust-lang.org/nomicon/subtyping.html#variance
//! [`PhantomData`]: https://doc.rust-lang.org/stable/std/marker/struct.PhantomData.html
//! [`Covariant`]: struct.Covariant.html
//! [`Contravariant`]: struct.Contravariant.html
//! [`Invariant`]: struct.Invariant.html

use core::marker::PhantomData;

/// A sealed trait implemented by `Covariant<T>`, `Contravariant<T>`, and
/// `Invariant<T>`.
pub trait Variance: Default + private::Sealed {}

/// Zero-sized type used to mark a type as [covariant] with respect to its type
/// parameter `T`.
///
/// [covariant]: https://en.wikipedia.org/wiki/Covariance_and_contravariance_(computer_science)
///
/// See the [module-level documentation](index.html) for more.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Covariant<T: ?Sized> {
    marker: PhantomData<fn() -> T>,
}

/// Zero-sized type used to mark a type as [contravariant] with respect to its type
/// parameter `T`.
///
/// [contravariant]: https://en.wikipedia.org/wiki/Covariance_and_contravariance_(computer_science)
///
/// See the [module-level documentation](index.html) for more.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Contravariant<T: ?Sized> {
    marker: PhantomData<fn(T)>,
}

/// Zero-sized type used to mark a type as [invariant] with respect to its type
/// parameter `T`.
///
/// [invariant]: https://en.wikipedia.org/wiki/Covariance_and_contravariance_(computer_science)
///
/// See the [module-level documentation](index.html) for more.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Invariant<T: ?Sized> {
    marker: (Covariant<T>, Contravariant<T>),
}

impl<T: ?Sized> Default for Covariant<T> {
    fn default() -> Self {
        Self { marker: Default::default(), }
    }
}
impl<T: ?Sized> Default for Contravariant<T> {
    fn default() -> Self {
        Self { marker: Default::default(), }
    }
}
impl<T: ?Sized> Default for Invariant<T> {
    fn default() -> Self {
        Self { marker: Default::default(), }
    }
}

impl<T: ?Sized> private::Sealed for Covariant<T> {}
impl<T: ?Sized> private::Sealed for Contravariant<T> {}
impl<T: ?Sized> private::Sealed for Invariant<T> {}

impl<T: ?Sized> Variance for Covariant<T> {}
impl<T: ?Sized> Variance for Contravariant<T> {}
impl<T: ?Sized> Variance for Invariant<T> {}

/// A convenience function for constructing any of `Covariant<T>`,
/// `Contravariant<T>`, and `Invariant<T>`. It is equivalent to [`default`].
///
/// [`default`]: https://doc.rust-lang.org/stable/std/default/trait.Default.html#tymethod.default
///
/// For example:
/// ```
/// use variance::{Covariant, variance};
///
/// struct Co<T> {
///     other_data: u32,
///     marker: Covariant<T>,
/// }
///
/// impl<T> Co<T> {
///     fn new() -> Self {
///         Co {
///             other_data: 42,
///             marker: variance(),
///         }
///     }
/// }
/// ```
pub fn variance<T: Variance>() -> T {
    Default::default()
}

// Prevent external implementations of `Variance`.
mod private {
    pub trait Sealed {}
}

// TODO: Implement real tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
