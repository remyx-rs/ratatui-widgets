//! The [`List`] widget is used to display a list of items and allows selecting one or multiple
//! items.

use alloc::vec::Vec;
use core::borrow::Borrow;
use core::hash::{Hash, Hasher};

use ratatui_core::layout::Rect;
use ratatui_core::style::{Style, Styled};
use ratatui_core::text::Line;
use strum::{Display, EnumString};

pub use self::item::ListItem;
pub use self::state::ListState;
use crate::block::{Block, BlockExt};
use crate::table::HighlightSpacing;

mod item;
mod rendering;
mod state;

/// A widget to display several items among which one can be selected (optional)
///
/// A list is a collection of [`ListItem`]s.
///
/// This is different from a [`Table`] because it does not handle columns, headers or footers and
/// the item's height is automatically determined. A `List` can also be put in reverse order (i.e.
/// *bottom to top*) whereas a [`Table`] cannot.
///
/// [`Table`]: crate::table::Table
///
/// List items can be aligned using [`Text::alignment`], for more details see [`ListItem`].
///
/// [`List`] is also a [`StatefulWidget`], which means you can use it with [`ListState`] to allow
/// the user to [scroll] through items and [select] one of them.
///
/// See the list in the [Examples] directory for a more in depth example of the various
/// configuration options and for how to handle state.
///
/// [Examples]: https://github.com/ratatui/ratatui/blob/main/examples/README.md
///
/// # Fluent setters
///
/// - [`List::highlight_style`] sets the style of the selected item.
/// - [`List::highlight_symbol`] sets the symbol to be displayed in front of the selected item.
/// - [`List::repeat_highlight_symbol`] sets whether to repeat the symbol and style over selected
///   multi-line items
/// - [`List::direction`] sets the list direction
///
/// # Examples
///
/// ```
/// use ratatui::Frame;
/// use ratatui::layout::Rect;
/// use ratatui::style::{Style, Stylize};
/// use ratatui::widgets::{Block, List, ListDirection, ListItem};
///
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
/// let items = ["Item 1", "Item 2", "Item 3"];
/// let list = List::new(items)
///     .block(Block::bordered().title("List"))
///     .style(Style::new().white())
///     .highlight_style(Style::new().italic())
///     .highlight_symbol(">>")
///     .repeat_highlight_symbol(true)
///     .direction(ListDirection::BottomToTop);
///
/// frame.render_widget(list, area);
/// # }
/// ```
///
/// # Stateful example
///
/// ```rust
/// use ratatui::Frame;
/// use ratatui::layout::Rect;
/// use ratatui::style::{Style, Stylize};
/// use ratatui::widgets::{Block, List, ListState};
///
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
/// // This should be stored outside of the function in your application state.
/// let mut state = ListState::default();
/// let items = ["Item 1", "Item 2", "Item 3"];
/// let list = List::new(items)
///     .block(Block::bordered().title("List"))
///     .highlight_style(Style::new().reversed())
///     .highlight_symbol(">>")
///     .repeat_highlight_symbol(true);
///
/// frame.render_stateful_widget(list, area, &mut state);
/// # }
/// ```
///
/// In addition to `List::new`, any iterator whose element is convertible to `ListItem` can be
/// collected into `List`.
///
/// ```
/// use ratatui::widgets::List;
///
/// (0..5).map(|i| format!("Item{i}")).collect::<List>();
/// ```
///
/// [`ListState`]: crate::list::ListState
/// [scroll]: crate::list::ListState::offset
/// [select]: crate::list::ListState::select
/// [`Text::alignment`]: ratatui_core::text::Text::alignment
/// [`StatefulWidget`]: ratatui_core::widgets::StatefulWidget
/// [`Widget`]: ratatui_core::widgets::Widget
#[derive(Debug, Clone)]
pub struct List<'a, Item = ListItem<'a>, Items = Vec<Item>, Message = ()>
where
    Items: Borrow<[Item]> + 'a,
    Item: PartialEq,
{
    /// An optional block to wrap the widget in
    pub(crate) block: Option<Block<'a>>,
    /// The items in the list
    pub(crate) items: Items,
    /// Precomputed index of the selected item
    pub(crate) selected: Option<usize>,
    /// Style used as a base style for the widget
    pub(crate) style: Style,
    /// List display direction
    pub(crate) direction: ListDirection,
    /// Style used to render selected item
    pub(crate) highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    pub(crate) highlight_symbol: Option<Line<'a>>,
    /// Whether to repeat the highlight symbol for each line of the selected item
    pub(crate) repeat_highlight_symbol: bool,
    /// Decides when to allocate spacing for the selection symbol
    pub(crate) highlight_spacing: HighlightSpacing,
    /// How many items to try to keep visible before and after the selected item
    pub(crate) scroll_padding: usize,
    /// Callback invoked when an item is selected
    pub(crate) on_select: fn(&Item) -> Message,
    /// Whether this widget currently has focus
    pub(crate) focus: bool,
}

impl<'a, Item, Items, Message> PartialEq for List<'a, Item, Items, Message>
where
    Items: Borrow<[Item]> + PartialEq + 'a,
    Item: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.block == other.block
            && self.items == other.items
            && self.selected == other.selected
            && self.style == other.style
            && self.direction == other.direction
            && self.highlight_style == other.highlight_style
            && self.highlight_symbol == other.highlight_symbol
            && self.repeat_highlight_symbol == other.repeat_highlight_symbol
            && self.highlight_spacing == other.highlight_spacing
            && self.scroll_padding == other.scroll_padding
            && self.focus == other.focus
    }
}

impl<'a, Item, Items, Message> Eq for List<'a, Item, Items, Message>
where
    Items: Borrow<[Item]> + Eq + 'a,
    Item: PartialEq + Eq,
{
}

impl<'a, Item, Items, Message> Hash for List<'a, Item, Items, Message>
where
    Items: Borrow<[Item]> + Hash + 'a,
    Item: PartialEq + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.block.hash(state);
        self.items.hash(state);
        self.selected.hash(state);
        self.style.hash(state);
        self.direction.hash(state);
        self.highlight_style.hash(state);
        self.highlight_symbol.hash(state);
        self.repeat_highlight_symbol.hash(state);
        self.highlight_spacing.hash(state);
        self.scroll_padding.hash(state);
    }
}

/// Defines the direction in which the list will be rendered.
///
/// If there are too few items to fill the screen, the list will stick to the starting edge.
///
/// See [`List::direction`].
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ListDirection {
    /// The first value is on the top, going to the bottom
    #[default]
    TopToBottom,
    /// The first value is on the bottom, going to the top.
    BottomToTop,
}

impl<'a, Item, Items, Message> List<'a, Item, Items, Message>
where
    Items: Borrow<[Item]> + 'a,
    Item: PartialEq,
{
    /// Creates a new list.
    ///
    /// The `items` parameter accepts any type that implements `Borrow<[Item]>`, such as
    /// `Vec<Item>`, `&[Item]`, or other collection types.
    ///
    /// The `selected` parameter is an optional reference to the currently selected item.
    /// Its index is computed by searching `items` for an equal element.
    ///
    /// The `on_select` callback is invoked when an item is selected.
    pub fn new<S>(items: Items, selected: Option<S>, on_select: fn(&Item) -> Message) -> Self
    where
        S: Borrow<Item>,
    {
        let selected_index =
            selected.and_then(|s| items.borrow().iter().position(|item| item == s.borrow()));
        Self {
            block: None,
            style: Style::default(),
            items,
            selected: selected_index,
            direction: ListDirection::default(),
            highlight_style: Style::default(),
            highlight_symbol: None,
            repeat_highlight_symbol: false,
            highlight_spacing: HighlightSpacing::default(),
            scroll_padding: 0,
            on_select,
            focus: false,
        }
    }

    /// Returns the area in which the list items are rendered.
    ///
    /// `area` is the full area the list is drawn into. The surrounding [`Block`] (if any) is
    /// subtracted, leaving only the region occupied by the items. This is the region that mouse
    /// clicks have to be mapped against to figure out which item was clicked.
    pub fn items_layout(&self, area: Rect) -> Rect {
        self.block.inner_if_some(area)
    }

    /// Returns the list items as a slice.
    pub fn items(&self) -> &[Item] {
        self.items.borrow()
    }

    /// Returns the current [`ListDirection`].
    pub fn direction_ref(&self) -> ListDirection {
        self.direction
    }

    /// Returns the precomputed index of the selected item, if any.
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Returns the current `on_select` callback.
    pub fn on_select_fn(&self) -> fn(&Item) -> Message {
        self.on_select
    }

    /// Wraps the list with a custom [`Block`] widget.
    ///
    /// The `block` parameter holds the specified [`Block`] to be created around the [`List`]
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Sets the base style of the widget
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// All text rendered by the widget will use this style, unless overridden by [`Block::style`],
    /// [`ListItem::style`], or the styles of the [`ListItem`]'s content.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Set the symbol to be displayed in front of the selected item
    ///
    /// By default there are no highlight symbol.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn highlight_symbol<L: Into<Line<'a>>>(mut self, highlight_symbol: L) -> Self {
        self.highlight_symbol = Some(highlight_symbol.into());
        self
    }

    /// Set the style of the selected item
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be applied to the entire item, including the
    /// [highlight symbol](List::highlight_symbol) if it is displayed, and will override any style
    /// set on the item or on the individual cells.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn highlight_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.highlight_style = style.into();
        self
    }

    /// Set whether to repeat the highlight symbol and style over selected multi-line items
    ///
    /// This is `false` by default.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn repeat_highlight_symbol(mut self, repeat: bool) -> Self {
        self.repeat_highlight_symbol = repeat;
        self
    }

    /// Set when to show the highlight spacing
    ///
    /// See [`HighlightSpacing`] for details on the available options.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn highlight_spacing(mut self, value: HighlightSpacing) -> Self {
        self.highlight_spacing = value;
        self
    }

    /// Defines the list direction (up or down)
    ///
    /// Defines if the `List` is displayed *top to bottom* (default) or *bottom to top*.
    /// If there is too few items to fill the screen, the list will stick to the starting edge.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn direction(mut self, direction: ListDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the number of items around the currently selected item that should be kept visible
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn scroll_padding(mut self, padding: usize) -> Self {
        self.scroll_padding = padding;
        self
    }

    /// Returns the number of items in the list
    pub fn len(&self) -> usize {
        self.items.borrow().len()
    }

    /// Returns true if the list contains no elements.
    pub fn is_empty(&self) -> bool {
        self.items.borrow().is_empty()
    }
}

impl<'a, Item, Items, Message> Styled for List<'a, Item, Items, Message>
where
    Items: Borrow<[Item]> + 'a,
    Item: PartialEq,
{
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl Styled for ListItem<'_> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use pretty_assertions::assert_eq;
    use ratatui_core::buffer::Buffer;
    use ratatui_core::layout::Rect;
    use ratatui_core::style::{Color, Modifier, Stylize};
    use ratatui_core::text::{Text, ToSpan};
    use ratatui_core::widgets::StatefulWidget;

    use super::*;

    fn noop(_: &ListItem) -> () {}

    #[test]
    fn can_be_stylized() {
        assert_eq!(
            List::new(vec![ListItem::new("")], None::<&ListItem>, noop)
                .black()
                .on_white()
                .bold()
                .not_dim()
                .style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::DIM)
        );
    }

    #[test]
    fn no_style() {
        let list = List::new(vec![ListItem::new("Item 1")], None::<&ListItem>, noop)
            .highlight_symbol(">>")
            .highlight_spacing(HighlightSpacing::Always);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));

        list.render(buffer.area, &mut buffer, &mut ListState::default());

        assert_eq!(buffer, Buffer::with_lines(["  Item 1  "]));
    }

    #[test]
    fn styled_text() {
        let list = List::new(
            vec![ListItem::new(Text::from("Item 1").bold())],
            None::<&ListItem>,
            noop,
        )
        .highlight_symbol(">>")
        .highlight_spacing(HighlightSpacing::Always);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));

        list.render(buffer.area, &mut buffer, &mut ListState::default());

        assert_eq!(
            buffer,
            Buffer::with_lines([Line::from(vec!["  ".to_span(), "Item 1  ".bold(),])])
        );
    }

    #[test]
    fn styled_list_item() {
        let item = ListItem::new("Item 1").style(Modifier::ITALIC);
        let list = List::new(vec![item.clone()], None::<&ListItem>, noop)
            .highlight_symbol(">>")
            .highlight_spacing(HighlightSpacing::Always);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));

        list.render(buffer.area, &mut buffer, &mut ListState::default());

        assert_eq!(
            buffer,
            Buffer::with_lines([Line::from_iter(["  Item 1  ".italic()])])
        );
    }

    #[test]
    fn styled_text_and_list_item() {
        let item = ListItem::new(Text::from("Item 1").bold()).style(Modifier::ITALIC);
        let list = List::new(vec![item.clone()], None::<&ListItem>, noop)
            .highlight_symbol(">>")
            .highlight_spacing(HighlightSpacing::Always);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));

        list.render(buffer.area, &mut buffer, &mut ListState::default());

        assert_eq!(
            buffer,
            Buffer::with_lines([Line::from(vec!["  ".italic(), "Item 1  ".bold().italic()])])
        );
    }

    #[test]
    fn styled_highlight() {
        let item = ListItem::new(Text::from("Item 1").bold()).style(Modifier::ITALIC);
        let list = List::new(vec![item.clone()], Some(&item), noop)
            .highlight_symbol(">>")
            .highlight_style(Color::Red);

        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        let mut state = ListState::default();
        list.render(buffer.area, &mut buffer, &mut state);

        assert_eq!(
            buffer,
            Buffer::with_lines([Line::from(vec![
                ">>".italic().red(),
                "Item 1  ".bold().italic().red(),
            ])])
        );
    }

    #[test]
    fn style_inheritance() {
        let bold = Modifier::BOLD;
        let italic = Modifier::ITALIC;
        let items = vec![
            ListItem::new(Text::raw("Item 1")),
            ListItem::new(Text::styled("Item 2", bold)),
            ListItem::new(Text::raw("Item 3")).style(italic),
            ListItem::new(Text::styled("Item 4", bold)).style(italic),
            ListItem::new(Text::styled("Item 5", bold)).style(italic),
        ];
        let list = List::new(
            items,
            Some(&ListItem::new(Text::styled("Item 5", bold)).style(italic)),
            noop,
        )
        .highlight_symbol(">>")
        .highlight_style(Color::Red)
        .style(Style::new().on_blue());

        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));
        let mut state = ListState::default();
        list.render(buffer.area, &mut buffer, &mut state);

        assert_eq!(
            buffer,
            Buffer::with_lines(vec![
                vec!["  Item 1  ".on_blue()],
                vec!["  ".on_blue(), "Item 2  ".bold().on_blue()],
                vec!["  Item 3  ".italic().on_blue()],
                vec![
                    "  ".italic().on_blue(),
                    "Item 4  ".bold().italic().on_blue(),
                ],
                vec![
                    ">>".italic().red().on_blue(),
                    "Item 5  ".bold().italic().red().on_blue(),
                ],
            ])
        );
    }

    #[test]
    fn render_in_minimal_buffer() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));
        let mut state = ListState::default();
        let list = List::new(
            vec![
                ListItem::new("Item 1"),
                ListItem::new("Item 2"),
                ListItem::new("Item 3"),
            ],
            None::<&ListItem>,
            noop,
        );
        list.render(buffer.area, &mut buffer, &mut state);
        assert_eq!(buffer, Buffer::with_lines(["I"]));
    }

    #[test]
    fn render_in_zero_size_buffer() {
        let mut buffer = Buffer::empty(Rect::ZERO);
        let mut state = ListState::default();
        let list = List::new(
            vec![
                ListItem::new("Item 1"),
                ListItem::new("Item 2"),
                ListItem::new("Item 3"),
            ],
            None::<&ListItem>,
            noop,
        );
        list.render(buffer.area, &mut buffer, &mut state);
    }
}
