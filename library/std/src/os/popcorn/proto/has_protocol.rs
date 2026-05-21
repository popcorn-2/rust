use super::ProtocolTuple;

#[marker]
pub trait HasProtocol<T> {}

// Ensures all protocols will appear on a RawHandle
impl<T> HasProtocol<T> for crate::os::popcorn::io::AnyProtocol {}

impl<T> HasProtocol<T> for T where Self: ProtocolTuple {}
#[cfg_attr(doc, doc(fake_variadic))]
impl<T> HasProtocol<T> for (T,) where Self: ProtocolTuple {}
#[doc(hidden)]
impl<T, U> HasProtocol<T> for (T, U) where Self: ProtocolTuple {}
#[doc(hidden)]
impl<T, U> HasProtocol<T> for (U, T) where Self: ProtocolTuple {}
#[doc(hidden)]
impl<T, U, V> HasProtocol<T> for (T, U, V) where Self: ProtocolTuple {}
#[doc(hidden)]
impl<T, U, V> HasProtocol<T> for (U, T, V) where Self: ProtocolTuple {}
#[doc(hidden)]
impl<T, U, V> HasProtocol<T> for (U, V, T) where Self: ProtocolTuple {}
#[doc(hidden)]
impl<T, U, V, W> HasProtocol<T> for (T, U, V, W) where Self: ProtocolTuple {}
#[doc(hidden)]
impl<T, U, V, W> HasProtocol<T> for (U, T, V, W) where Self: ProtocolTuple {}
#[doc(hidden)]
impl<T, U, V, W> HasProtocol<T> for (U, V, T, W) where Self: ProtocolTuple {}
#[doc(hidden)]
impl<T, U, V, W> HasProtocol<T> for (U, V, W, T) where Self: ProtocolTuple {}
#[doc(hidden)]
impl<T, U, V, W, X> HasProtocol<T> for (T, U, V, W, X) where Self: ProtocolTuple {}
#[doc(hidden)]
impl<T, U, V, W, X> HasProtocol<T> for (U, T, V, W, X) where Self: ProtocolTuple {}
#[doc(hidden)]
impl<T, U, V, W, X> HasProtocol<T> for (U, V, T, W, X) where Self: ProtocolTuple {}
#[doc(hidden)]
impl<T, U, V, W, X> HasProtocol<T> for (U, V, W, T, X) where Self: ProtocolTuple {}
#[doc(hidden)]
impl<T, U, V, W, X> HasProtocol<T> for (U, V, W, X, T) where Self: ProtocolTuple {}
