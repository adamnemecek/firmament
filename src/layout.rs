use crate::{Length, Node, Limits};
pub struct Layout {
    /// Returns the width of the [`Widget`].
    ///
    /// [`Widget`]: trait.Widget.html
    width: Length,

    /// Returns the height of the [`Widget`].
    ///
    /// [`Widget`]: trait.Widget.html
    height: Length,

    /// Returns the [`Node`] of the [`Widget`].
    ///
    /// This [`Node`] is used by the runtime to compute the [`Layout`] of the
    /// user interface.
    ///
    /// [`Node`]: ../layout/struct.Node.html
    /// [`Widget`]: trait.Widget.html
    /// [`Layout`]: ../layout/struct.Layout.html

    layout_fn: fn(&Limits) -> Node,
    // fn layout(
    //     &self,
    //     // renderer: &Renderer,
    //     limits: &Limits,
    // ) -> Node;

    // /// Draws the [`Widget`] using the associated `Renderer`.
    // ///
    // /// [`Widget`]: trait.Widget.html
    // fn draw(
    //     &self,
    //     renderer: &mut Renderer,
    //     defaults: &Renderer::Defaults,
    //     layout: Layout<'_>,
    //     cursor_position: Point,
    // ) -> Renderer::Output;

    // /// Computes the _layout_ hash of the [`Widget`].
    // ///
    // /// The produced hash is used by the runtime to decide if the [`Layout`]
    // /// needs to be recomputed between frames. Therefore, to ensure maximum
    // /// efficiency, the hash should only be affected by the properties of the
    // /// [`Widget`] that can affect layouting.
    // ///
    // /// For example, the [`Text`] widget does not hash its color property, as
    // /// its value cannot affect the overall [`Layout`] of the user interface.
    // ///
    // /// [`Widget`]: trait.Widget.html
    // /// [`Layout`]: ../layout/struct.Layout.html
    // /// [`Text`]: text/struct.Text.html
    // fn hash_layout(&self, state: &mut Hasher);

    // /// Processes a runtime [`Event`].
    // ///
    // /// It receives:
    // ///   * an [`Event`] describing user interaction
    // ///   * the computed [`Layout`] of the [`Widget`]
    // ///   * the current cursor position
    // ///   * a mutable `Message` list, allowing the [`Widget`] to produce
    // ///   new messages based on user interaction.
    // ///   * the `Renderer`
    // ///   * a [`Clipboard`], if available
    // ///
    // /// By default, it does nothing.
    // ///
    // /// [`Event`]: ../enum.Event.html
    // /// [`Widget`]: trait.Widget.html
    // /// [`Layout`]: ../layout/struct.Layout.html
    // /// [`Clipboard`]: ../trait.Clipboard.html
    // fn on_event(
    //     &mut self,
    //     _event: Event,
    //     _layout: Layout<'_>,
    //     _cursor_position: Point,
    //     _messages: &mut Vec<Message>,
    //     _renderer: &Renderer,
    //     _clipboard: Option<&dyn Clipboard>,
    // ) {
    // }

    // /// Returns the overlay of the [`Element`], if there is any.
    // ///
    // /// [`Element`]: struct.Element.html
    // fn overlay(
    //     &mut self,
    //     _layout: Layout<'_>,
    // ) -> Option<overlay::Element<'_, Message, Renderer>> {
    //     None
    // }
}

impl Layout {
    pub fn width(&self) -> Length {
        self.width
    }

    pub fn height(&self) -> Length {
        self.height
    }

    pub fn layout(&self, limits: &Limits) -> Node {
        (self.layout_fn)(limits)
    }
}