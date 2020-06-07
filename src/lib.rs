pub mod parser;
use std::collections::HashMap;

// TODO: wrap the useful variables into their own module
//
/// The type alias for clarity to the name of the variable
type VariableName<'a> = &'a str;

/// Type alias for a variable
/// In essence the name of the varaible as its type
type Variable<'a> = (VariableName<'a>, VariableType);

/// Type alias for a vector of actions
type Sequence<'a> = Vec<Action<'a>>;

/// # Statement types
/// These are the possible statements in the language
#[derive(Debug)]
pub enum StatementType<'a> {
    Trigger(&'a str),
}

/// # Type System
/// Currently the only possible types for variables are
/// either a sequence or a singular action.
#[derive(Debug)]
pub enum VariableType {
    Seq,
    Act,
}

/// This is required to convert the text to possible types
/// TODO: move this conversion into our parsers
impl From<&str> for VariableType {
    fn from(i: &str) -> VariableType {
        match i {
            "seq" => VariableType::Seq,
            "act" => VariableType::Act,
            _ => panic!("Unknown type"),
        }
    }
}

/// # Collection of entities
/// These entities need to be understood and resolved down the line
/// Esentially this is the AST like object
type Entities<'a> = Vec<Entity<'a>>;

/// # Code based Entities
/// This enum specifies wether something is a statement,
/// an assigned variable or a comment (WIP)
#[derive(Debug)]
pub enum Entity<'a> {
    AssignedSequence((Variable<'a>, Sequence<'a>)),
    AssignedAction((Variable<'a>, Action<'a>)),
    Statement(StatementType<'a>),
}

/// # Actions
/// Actions are possible command types that the interpreter
/// can send to the bridge
#[derive(Debug, Clone)]
pub enum Action<'a> {
    Wait(u16),
    Blink((u16, u16, &'a str)),
    Color(&'a str),
}

/// Executor for any action that it gets passed
pub fn execute_statement(action: &Entity) {
    use Action::*;
    use Entity::*;

    let interpret_action = |action: &Action| match action {
        Wait(duration) => println!("Waiting for {}", duration),
        Blink((speed, pause, color)) => println!(
            "Blinking - speed: {}, pause:{} and color: {}",
            speed, pause, color
        ),
        Color(color) => println!("Changing color to {}", color),
        _ => println!("Unknown command"),
    };
    let interpret_sequence = |sequence: &Vec<Action>| {
        for action in sequence.iter() {
            interpret_action(action)
        }
    };

    match action {
        AssignedAction(((_, _), action)) => interpret_action(action),
        AssignedSequence(((_, _), sequence)) => interpret_sequence(sequence),
        Statement(_) => panic!("Don't know whats up!?"),
    }
}

// TODO: use the assigned actions and sequences to store them rather than unfolding them
// TODO: this needs to panic if an undefined variable is used
// TODO: needs to panic in parsers - sort out proper error reporting
/// Get all variables into a hashmap from their their name
/// to their entity.
pub fn runtime(entities: Entities) {
    use Entity::*;
    use StatementType::*;
    let mut variables = HashMap::new();
    for entity in entities {
        match entity {
            AssignedAction(((name, _), _)) => {
                variables.insert(name, entity);
            }
            AssignedSequence(((name, _), _)) => {
                variables.insert(name, entity);
            }
            Statement(Trigger(which)) => execute_statement(&variables[which]),
        }
    }
}
