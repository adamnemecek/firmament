// pub struct 
/// Alignment on an axis of a container.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Align {
    /// Align at the start of the axis.
    Start,

    /// Align at the center of the axis.
    Center,

    /// Align at the end of the axis.
    End,
}