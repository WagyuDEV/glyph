use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub use move_command::{Directions, MoveCommand};
pub use quit_command::QuitCommand;

use crate::state::State;

mod move_command;
mod quit_command;

#[derive(Hash, Eq, PartialEq)]
pub enum EditorCommands {
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

pub trait Command {
    fn execute(&self);
}

pub struct Commands {}

impl Commands {
    pub fn make_commands(state: Rc<RefCell<State>>) -> HashMap<EditorCommands, Box<dyn Command>> {
        let mut command_map: HashMap<EditorCommands, Box<dyn Command>> = HashMap::new();
        Commands::make_actions_commands(Rc::clone(&state), &mut command_map);
        Commands::make_move_commands(Rc::clone(&state), &mut command_map);
        command_map
    }

    fn make_actions_commands(
        state: Rc<RefCell<State>>,
        command_map: &mut HashMap<EditorCommands, Box<dyn Command>>,
    ) {
        command_map.insert(
            EditorCommands::Quit,
            Box::new(QuitCommand::new(Rc::clone(&state))),
        );
    }

    fn make_move_commands(
        state: Rc<RefCell<State>>,
        command_map: &mut HashMap<EditorCommands, Box<dyn Command>>,
    ) {
        command_map.insert(
            EditorCommands::MoveUp,
            Box::new(MoveCommand::new(Rc::clone(&state), Directions::Up)),
        );
        command_map.insert(
            EditorCommands::MoveDown,
            Box::new(MoveCommand::new(Rc::clone(&state), Directions::Down)),
        );
        command_map.insert(
            EditorCommands::MoveLeft,
            Box::new(MoveCommand::new(Rc::clone(&state), Directions::Left)),
        );
        command_map.insert(
            EditorCommands::MoveRight,
            Box::new(MoveCommand::new(Rc::clone(&state), Directions::Right)),
        );
    }
}
