use crate::{App, InputMode, Pokidex};


pub fn new_pokientry(app: &mut App){
    app.input_mode = InputMode::Adding;
    app.selected_pokientry = None;
}

pub fn exit_app(app: &mut App){
    app.should_exit = true;
}

pub fn list_up(app: &mut App) {
    if let Some(index) = app.selected_pokientry {
        if index > 0 {
            app.selected_pokientry = Some(index - 1);
        }
    }
}

pub fn list_down(app: &mut App) {
    if let Some(index) = app.selected_pokientry {
        if index < app.pokientrys.len() - 1 {
            app.selected_pokientry = Some(index + 1);
        }
    }
}

pub fn check_and_uncheck_pokientry(app: &mut App) {
    if let Some(index) = app.selected_pokientry {
        app.pokientrys[index].completed = !app.pokientrys[index].completed;
    }
}

pub fn delete_pokientry(app: &mut App) {
    if let Some(index) = app.selected_pokientry {
        app.pokientrys.remove(index);
        if app.pokientrys.is_empty() {
            app.selected_pokientry = None;
        } else if app.selected_pokientry.unwrap() == app.pokientrys.len() {
            app.selected_pokientry = Some(app.pokientrys.len() - 1);
        }
    }
}

pub fn save_pokientry(app: &mut App) {
    let new_pokientry = Pokidex::new(app.input.drain(..).collect());
    app.pokientrys.push(new_pokientry);
}

pub fn exit_adding(app: &mut App) {
    app.input_mode = InputMode::Normal;
    app.selected_pokientry = Some(0);
}

pub fn input_add_char(app: &mut App, c: char) {
    app.input.push(c);
}

pub fn input_del_char(app: &mut App) {
    app.input.pop();
}
