//! Marker trait automatically implemented by types that provide a matching layout for Julia data.
use crate::{convert::unbox::Unbox, data::layout::valid_layout::ValidLayout};

/// Marker trait automatically implemented by types that provide a matching layout for Julia data.
pub trait InlineLayout: ValidLayout + Unbox<Output = Self> {}

impl<T: ValidLayout + Unbox<Output = Self>> InlineLayout for T {}