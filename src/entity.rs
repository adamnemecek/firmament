use std::num::NonZeroI32;

/// An index is basically the id of an `Entity`.
pub type Index = u32;

/// Index generation. When a new entity is placed at an old index,
/// it bumps the `Generation` by 1. This allows to avoid using components
/// from the entities that were deleted.
#[derive(Clone, Copy, Hash, Eq, Ord, Debug, PartialEq, PartialOrd)]
pub struct Generation(NonZeroI32);

/// `Entity` type, as seen by the user.
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Entity(Index, Generation);

