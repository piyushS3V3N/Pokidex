// events.rs
use crate::{actions, App, InputMode, TerminalFrame};
use crossterm::event::{KeyCode, KeyEvent};
use tui::layout::Rect;

/// Input events handler
pub fn handle_events(event: KeyEvent, app: &mut App) {
    match app.input_mode {
        InputMode::Normal => handle_normal_events(app, event.code),
        InputMode::Adding => handle_adding_events(app, event.code),
    }
}

/// When user is viewing pokientrys
fn handle_normal_events(app: &mut App, keycode: KeyCode) {
    let keybindings = &app.configs.keybindings;

    match keycode {
        KeyCode::Char('q') => app.should_exit = true,
        KeyCode::Char('i') => app.show_image = true,
        KeyCode::Char('j') => {
            if let Some(index) = app.selected_pokientry {
                if index + 1 < app.pokientrys.len() {
                    app.selected_pokientry = Some(index + 1);
                }
            } else {
                app.selected_pokientry = Some(0);
            }
        }
        KeyCode::Char('k') => {
            if let Some(index) = app.selected_pokientry {
                if index > 0 {
                    app.selected_pokientry = Some(index - 1);
                }
            }
        }
        code if code == keybindings.new_pokientry => actions::new_pokientry(app),
        code if code == keybindings.exit_app => actions::exit_app(app),
        code if code == keybindings.list_up => actions::list_up(app),
        code if code == keybindings.list_down => actions::list_down(app),
        code if code == keybindings.check_and_uncheck_pokientry => actions::check_and_uncheck_pokientry(app),
        code if code == keybindings.delete_pokientry => actions::delete_pokientry(app),
        _ => {}
    }
}

/// When user adding a new pokientry
fn handle_adding_events(app: &mut App, keycode: KeyCode) {
    let keybindings = &app.configs.keybindings;

    match keycode {
        code if code == keybindings.save_pokientry && !app.input.trim().is_empty() => {
            actions::save_pokientry(app);
        }
        code if code == keybindings.exit_adding => actions::exit_adding(app),
        KeyCode::Char(c) => actions::input_add_char(app, c),
        KeyCode::Backspace => actions::input_del_char(app),
        _ => {}
    }
}

/// Handle cursor when typing
pub fn handle_input_cursor(app: &App, frame: &mut TerminalFrame, chunks: &[Rect]) {
    match app.input_mode {
        InputMode::Normal => (),
        InputMode::Adding => {
            frame.set_cursor(
                chunks[1].x + app.input.len() as u16 + 1,
                chunks[1].y + 1,
            )
        }
    }
}

