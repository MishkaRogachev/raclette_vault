use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, MouseButton, MouseEventKind};

pub trait Focusable {
    fn is_focused(&self) -> bool;
    fn set_focused(&mut self, focused: bool);
    fn contains(&self, column: u16, row: u16) -> bool;
    fn handle_event(&mut self, event: &Event);
}

pub fn handle_scoped_event(focusables: &mut [&mut dyn Focusable], event: &Event) -> bool {
    // 1. Find the focused widget
    let mut focused_index: Option<usize> = None;
    for (i, widget) in focusables.iter_mut().enumerate() {
        if widget.is_focused() {
            focused_index = Some(i);
            break;
        }
    }

    match event {
        // 2. If tab is pressed, unfocus the current widget and focus the next one
        Event::Key(KeyEvent { code: KeyCode::Tab, .. }) => {
            if let Some(index) = focused_index {
                // Unfocus the current widget
                focusables[index].set_focused(false);

                // 2.1. If focused widget is last, unfocus it (no focus)
                if index + 1 >= focusables.len() {
                    return true;
                }

                // Focus the next widget
                focusables[index + 1].set_focused(true);
            } else {
                // 2.2. If no focused widget, focus the first one
                if !focusables.is_empty() {
                    focusables[0].set_focused(true);
                }
            }
            return true;
        }

        // 3. If esc is pressed, unfocus the current widget
        Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => {
            if let Some(index) = focused_index {
                focusables[index].set_focused(false); // Unfocus the current widget
            }
            return true;
        }

        // 4. If mouse click is pressed, focus the widget that was clicked
        Event::Mouse(mouse_event) => {
            if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                for widget in focusables.iter_mut() {
                    widget.set_focused(widget.contains(mouse_event.column, mouse_event.row));
                }
            }
        }
        _ => {}
    }

    // 5. If there is a focused widget, call handle_event on it
    if let Some(index) = focused_index {
        focusables[index].handle_event(event);
        return true;
    }
    return false;
}
