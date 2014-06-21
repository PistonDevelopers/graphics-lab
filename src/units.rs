
//! Provides safe unit abstractions.

/// Wraps a number with a unit as phantom type.
pub struct Number<T, Unit>(pub T);


