//! Button widget implementation

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::Widget,
};

/// Button state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    /// Normal state
    Normal,
    /// Button is hovered/selected
    Selected,
    /// Button is being pressed/activated
    Active,
}

/// A button widget with customizable style
#[derive(Debug, Clone)]
pub struct Button<'a> {
    /// The content of the button
    content: Text<'a>,
    /// The current state of the button
    state: ButtonState,
    /// Style for normal state
    normal_style: Style,
    /// Style for selected/hover state
    selected_style: Style,
    /// Style for active/pressed state
    active_style: Style,
}

impl<'a> Button<'a> {
    /// Create a new button with the given content
    pub fn new<T: Into<Text<'a>>>(content: T) -> Self {
        Self {
            content: content.into(),
            state: ButtonState::Normal,
            normal_style: Style::default().fg(Color::Yellow).bg(Color::DarkGray),
            selected_style: Style::default().fg(Color::White).bg(Color::DarkGray),
            active_style: Style::default().fg(Color::Black).bg(Color::Yellow),
        }
    }

    /// Set the button state
    pub fn state(mut self, state: ButtonState) -> Self {
        self.state = state;
        self
    }

    /// Set the normal style
    pub fn normal_style(mut self, style: Style) -> Self {
        self.normal_style = style;
        self
    }

    /// Set the selected/hover style
    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    /// Set the active/pressed style
    pub fn active_style(mut self, style: Style) -> Self {
        self.active_style = style;
        self
    }

    /// Set the button state to normal
    pub fn normal(&mut self) {
        self.state = ButtonState::Normal;
    }

    /// Set the button state to selected/hover
    pub fn select(&mut self) {
        self.state = ButtonState::Selected;
    }

    /// Set the button state to active/pressed
    pub fn press(&mut self) {
        self.state = ButtonState::Active;
    }

    /// Get the current style based on the button state
    fn current_style(&self) -> Style {
        match self.state {
            ButtonState::Normal => self.normal_style,
            ButtonState::Selected => self.selected_style,
            ButtonState::Active => self.active_style,
        }
    }

    /// Get the content as a string
    pub fn content_str(&self) -> String {
        // Convert the Text content to a string
        self.content.lines.iter()
            .flat_map(|line| line.spans.iter())
            .map(|span| span.content.clone())
            .collect()
    }
}

impl<'a> Widget for Button<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Get the style for the current state
        let style = self.current_style();

        // Apply the style to the entire button area
        buf.set_style(area, style);

        // Draw a border around the button to make it more visible
        for x in area.x..area.x + area.width {
            buf.get_mut(x, area.y).set_symbol("─").set_style(style);
            buf.get_mut(x, area.y + area.height - 1).set_symbol("─").set_style(style);
        }

        for y in area.y..area.y + area.height {
            buf.get_mut(area.x, y).set_symbol("│").set_style(style);
            buf.get_mut(area.x + area.width - 1, y).set_symbol("│").set_style(style);
        }

        // Draw corners
        buf.get_mut(area.x, area.y).set_symbol("┌").set_style(style);
        buf.get_mut(area.x + area.width - 1, area.y).set_symbol("┐").set_style(style);
        buf.get_mut(area.x, area.y + area.height - 1).set_symbol("└").set_style(style);
        buf.get_mut(area.x + area.width - 1, area.y + area.height - 1).set_symbol("┘").set_style(style);

        // Calculate the position to center the content
        let width = self.content.width() as u16;
        let x = area.x + (area.width.saturating_sub(width)) / 2;
        let y = area.y + area.height / 2;

        // Render the content
        buf.set_spans(x, y, self.content.lines.first().unwrap_or_default().clone(), width);
    }
}

