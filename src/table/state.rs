/// State of a [`Table`] widget
///
/// This state tracks the scroll offset and table length.
/// Row selection is managed by the [`Table`] widget itself via the `selected`
/// parameter passed to [`Table::new`].
///
/// [`Table`]: super::Table
/// [`Table::new`]: super::Table::new
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TableState {
    pub(crate) offset: usize,
    pub(crate) length: usize,
}

impl TableState {
    /// Creates a new [`TableState`]
    pub const fn new(length: usize) -> Self {
        Self { offset: 0, length }
    }

    /// Sets the index of the first row to be displayed
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Returns the total number of rows in the table.
    pub const fn len(&self) -> usize {
        self.length
    }

    /// Returns `true` if the table has no rows.
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Index of the first row to be displayed
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Mutable reference to the index of the first row to be displayed
    pub const fn offset_mut(&mut self) -> &mut usize {
        &mut self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let state = TableState::new(10);
        assert_eq!(state.offset(), 0);
        assert_eq!(state.len(), 10);
    }

    #[test]
    fn default_state() {
        let state = TableState::default();
        assert_eq!(state.offset(), 0);
        assert_eq!(state.len(), 0);
        assert!(state.is_empty());
    }

    #[test]
    fn with_offset() {
        let state = TableState::default().with_offset(1);
        assert_eq!(state.offset, 1);
    }
}
