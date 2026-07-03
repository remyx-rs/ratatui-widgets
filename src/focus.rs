//! Focus management for stateful widgets.
//!
//! This module provides the [`Focusable`] trait, which allows widgets to track
//! whether they currently have input focus. Implementations are provided for
//! [`List`] and [`Table`].

use core::borrow::Borrow;

use crate::{list::List, paragraph::Paragraph, table::Table};

/// A widget that can receive and report input focus.
pub trait Focusable {
    /// Sets the focus state and returns the modified widget.
    fn focus(self, focus: bool) -> Self;

    /// Returns `true` if the widget currently has focus.
    fn is_focused(&self) -> bool;
}

impl<'a, Item, Items, Message> Focusable for List<'a, Item, Items, Message>
where
    Items: Borrow<[Item]> + 'a,
    Item: PartialEq,
{
    fn focus(mut self, focus: bool) -> Self {
        self.focus = focus;
        self
    }

    fn is_focused(&self) -> bool {
        self.focus
    }
}

impl<'a, Item, Items, Message> Focusable for Table<'a, Item, Items, Message>
where
    Items: Borrow<[Item]> + 'a,
    Item: PartialEq,
{
    fn focus(mut self, focus: bool) -> Self {
        self.focus = focus;
        self
    }

    fn is_focused(&self) -> bool {
        self.focus
    }
}

impl<'a> Focusable for Paragraph<'a> {
    fn focus(mut self, focus: bool) -> Self {
        self.focus = focus;
        self
    }

    fn is_focused(&self) -> bool {
        self.focus
    }
}
