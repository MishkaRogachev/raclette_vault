use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, MouseButton, MouseEventKind};

pub enum FocusableEvent {
    FocusChanged,
    FocusFinished,
    Input(String),
    Enter,
}

pub trait Focusable {
    fn is_focused(&self) -> bool;
    fn set_focused(&mut self, focused: bool);
    fn contains(&self, column: u16, row: u16) -> bool;
    fn handle_event(&mut self, event: &Event) -> Option<FocusableEvent>;
}

pub fn handle_scoped_event(focusables: &mut [&mut dyn Focusable], event: &Event) -> Option<FocusableEvent> {
    // Find the focused widget
    let mut focused_index: Option<usize> = None;
    for (i, widget) in focusables.iter_mut().enumerate() {
        if widget.is_focused() {
            focused_index = Some(i);
            break;
        }
    }

    match event {
        Event::Key(KeyEvent { code: KeyCode::Tab, .. }) => {
            if let Some(index) = focused_index {
                // Unfocus the current widget
                focusables[index].set_focused(false);

                // If focused widget is last, unfocus it (no focus)
                if index + 1 >= focusables.len() {
                    return Some(FocusableEvent::FocusChanged);
                }

                // Focus the next widget
                focusables[index + 1].set_focused(true);
            } else {
                // If no focused widget, focus the first one
                if !focusables.is_empty() {
                    focusables[0].set_focused(true);
                }
            }
            return Some(FocusableEvent::FocusChanged);
        }

        // If esc is pressed, unfocus the current widget
        Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => {
            if let Some(index) = focused_index {
                focusables[index].set_focused(false); // Unfocus the current widget
            }
            return Some(FocusableEvent::FocusChanged);
        }

        // If mouse click is pressed, focus the widget that was clicked
        Event::Mouse(mouse_event) => {
            if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                for widget in focusables.iter_mut() {
                    widget.set_focused(widget.contains(mouse_event.column, mouse_event.row));
                }
            }
        }
        _ => {}
    }

    // If there is a focused widget, call handle_event on it
    if let Some(index) = focused_index {
        if let Some(event) = focusables[index].handle_event(event) {
            match event {
                FocusableEvent::Enter => {
                    // Enter is pressed, unfocus the current widget and focus the next one
                    focusables[index].set_focused(false);
                    if index + 1 < focusables.len() {
                        focusables[index + 1].set_focused(true);
                        return Some(FocusableEvent::FocusChanged);
                    }
                    return Some(FocusableEvent::FocusFinished);
                }
                _ => { return Some(event); }
            }
        }
    }
    return None;
}
