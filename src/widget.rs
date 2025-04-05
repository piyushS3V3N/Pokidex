//! Custom widgets

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
};

use crate::{configs::keycode_to_string, App, InputMode, Pokidex};



/// Split terminal view
pub fn main_chunks(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(3),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(area);

    chunks
}

/// Shows a list of pokientrys
pub fn pokientry_list(app: &App) -> List {
    // Map pokientrys to ListItem widget
    let pokientrys: Vec<ListItem> = app
        .pokientrys
        .iter()
        .enumerate()
        .map(|q| indexed_pokientry_item(app, q))
        .collect();

    List::new(pokientrys).style(app.default_style()).block(
        Block::default()
            .title(" Work To Do :') ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(app.default_style()),
    )
}

/// Check if a pokientry is selected then renders it properly
fn indexed_pokientry_item<'a>(app: &'a App, (index, pokientry): (usize, &Pokidex)) -> ListItem<'a> {
    if let Some(selected_index) = app.selected_pokientry {
        pokientry_item(
            pokientry.title.clone(),
            pokientry.completed,
            selected_index == index,
            app,
        )
    } else {
        pokientry_item(pokientry.title.clone(), pokientry.completed, false, app)
    }
}

/// Widget to show a single pokientry
fn pokientry_item(title: String, completed: bool, selected: bool, app: &App) -> ListItem {
    let style = if selected {
        if app.blink_state {
            app.selection_style().add_modifier(Modifier::BOLD)
        } else {
            app.selection_style()
        }
    } else {
        app.default_style()
    };

    let pokientry = if completed {
        ListItem::new(Spans::from(vec![
            Span::styled("✔  ", app.check_sign_style(selected)),
            Span::styled(title, app.checked_pokientry_style(selected)),
        ]))
    } else {
        ListItem::new(Spans::from(vec![
            Span::styled("   ", style),
            Span::styled(title, style),
        ]))
    };

    pokientry.style(style)
}
/// Input field to make a new pokientry
pub fn pokientry_input(app: &App) -> Paragraph {
    let style = match app.input_mode {
        InputMode::Normal => app.default_style(),
        InputMode::Adding => app.default_style().fg(app.configs.colors.selection_bg),
    };

    let input = Paragraph::new(app.input.as_ref()).style(style).block(
        Block::default()
            .borders(Borders::ALL)
            .title("New Thing You Forgot ToDo")
            .border_type(BorderType::Rounded)
            .style(style),
    );

    input
}

/// Help text
pub fn navigation_hint(app: &App) -> Paragraph {
    let keybindings = &app.configs.keybindings;

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::styled(
                    keycode_to_string(keybindings.exit_app),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" exit | ", app.default_style()),
                Span::styled(
                    keycode_to_string(keybindings.new_pokientry),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" new pokientry | ", app.default_style()),
                Span::styled(
                    keycode_to_string(keybindings.check_and_uncheck_pokientry),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" check/uncheck pokientry | ", app.default_style()),
                Span::styled(
                    format!(
                        "{}/{}",
                        keycode_to_string(keybindings.list_up),
                        keycode_to_string(keybindings.list_down)
                    ),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" navigate list | ", app.default_style()),
                Span::styled(
                    keycode_to_string(keybindings.delete_pokientry),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" delete pokientry", app.default_style()),
            ],
            app.default_style().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Adding => (
            vec![
                Span::styled(
                    keycode_to_string(keybindings.exit_adding),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" stop adding | ", app.default_style()),
                Span::styled(
                    keycode_to_string(keybindings.save_pokientry),
                    app.default_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(" save pokientry", app.default_style()),
            ],
            app.default_style(),
        ),
    };
    let mut help_text = Text::from(Spans::from(msg));
    help_text.patch_style(style);
    Paragraph::new(help_text).style(app.default_style())
}
