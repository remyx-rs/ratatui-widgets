/// State of the [`List`] widget
///
/// This state tracks the scroll offset and list length. Selection is managed by the
/// [`List`] widget itself via the `selected` parameter passed to [`List::new`].
///
/// The state consists of:
/// - [`offset`]: the index of the first item to be displayed
/// - [`length`]: the total number of items in the list, managed externally by the caller
///
/// [`offset`]: ListState::offset()
/// [`length`]: ListState::len()
///
/// [`List`]: super::List
/// [`List::new`]: super::List::new
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListState {
    pub(crate) offset: usize,
    pub(crate) length: usize,
}

impl ListState {
    /// Creates a new [`ListState`] with the given list length.
    ///
    /// The `length` is managed externally by the caller and is not updated by the widget
    /// during rendering.
    pub const fn new(length: usize) -> Self {
        Self { offset: 0, length }
    }

    /// Sets the index of the first item to be displayed
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Returns the total number of items in the list, as set by the caller.
    pub const fn len(&self) -> usize {
        self.length
    }

    /// Returns `true` if the list has no items.
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Index of the first item to be displayed
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Mutable reference to the index of the first item to be displayed
    pub const fn offset_mut(&mut self) -> &mut usize {
        &mut self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let state = ListState::new(10);
        assert_eq!(state.offset(), 0);
        assert_eq!(state.len(), 10);
    }

    #[test]
    fn default_state() {
        let state = ListState::default();
        assert_eq!(state.offset(), 0);
        assert_eq!(state.len(), 0);
        assert!(state.is_empty());
    }

    #[test]
    fn with_offset() {
        let state = ListState::default().with_offset(5);
        assert_eq!(state.offset(), 5);
    }

    #[test]
    fn offset_mut() {
        let mut state = ListState::default();
        *state.offset_mut() = 3;
        assert_eq!(state.offset(), 3);
    }
}
