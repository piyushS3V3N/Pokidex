use crossterm::{
    event::{read, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pokidex_tui::{
    events::{handle_events, handle_input_cursor},
    file_handler::{load_configs, load_pokientrys, save_pokientrys},
    widget, App, CrossTerminal, DynResult, TerminalFrame,
};
use std::{error::Error, io::stdout};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> DynResult {
    let mut terminal = initialize_terminal()?;

    let pokientrys = load_pokientrys()?;
    let configs = load_configs()?;
    let mut app = App::new(&pokientrys, configs);

    draw_ui(&mut terminal, &mut app)?;
    cleanup_terminal(terminal)?;
    save_pokientrys(&app.pokientrys)?;

    Ok(())
}

/// Setup terminal
fn initialize_terminal() -> Result<CrossTerminal, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    Ok(terminal)
}

/// Draw user interface to terminal
fn draw_ui(terminal: &mut CrossTerminal, app: &mut App) -> DynResult {
    while !app.should_exit {
        terminal.draw(|f| {
            app_view(f, app);
        })?;

        if let Event::Key(event) = read()? {
            handle_events(event, app);
        }
    }

    Ok(())
}

/// Return terminal to it's normal state
fn cleanup_terminal(mut terminal: CrossTerminal) -> DynResult {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

/// A single frame of application view
fn app_view(frame: &mut TerminalFrame, app: &App) {
    let main_chunks = widget::main_chunks(frame.size());

    let pokientry_list = widget::pokientry_list(app);
    frame.render_widget(pokientry_list, main_chunks[0]);

    let pokientry_input = widget::pokientry_input(app);
    frame.render_widget(pokientry_input, main_chunks[1]);
    handle_input_cursor(&app, frame, &main_chunks);

    let navigation_hint = widget::navigation_hint(app);
    frame.render_widget(navigation_hint, main_chunks[2]);
}
