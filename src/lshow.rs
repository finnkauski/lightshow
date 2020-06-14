use super::midi::Pad;
use lighthouse::{hue::HueBridge, state};
use std::collections::HashMap;

// TODO: wrap the useful variables into their own module
/// The type alias for clarity to the name of the variable
pub type VariableName<'a> = &'a str;

/// Type alias for a variable
/// In essence the name of the varaible as its type
pub type Variable<'a> = (VariableName<'a>, VariableType);

/// Type alias for multiple parsed variables
pub type Variables<'a> = HashMap<VariableName<'a>, Entity<'a>>;

/// Type alias for multiple parsed variables
/// Allows to bind sequences to midi triggers
pub type MidiBinds<'a> = HashMap<Pad, VariableName<'a>>;

/// Type alias for a vector of actions
pub type Sequence<'a> = Vec<Action<'a>>;

/// # Statement types
/// These are the possible statements in the language
#[derive(Debug)]
pub enum StatementType<'a> {
    Trigger(&'a str),
}

/// # DirectiveType
/// File level directives that change the behaviour of the language
#[derive(Debug, Eq, PartialEq)]
pub enum DirectiveType {
    Midi,
}

/// This is required to convert the text to possible directive types
/// TODO: move this conversion into our parsers
impl From<&str> for DirectiveType {
    fn from(i: &str) -> DirectiveType {
        match i {
            "midi" => DirectiveType::Midi,
            _ => panic!("Unknown directive"),
        }
    }
}

/// Directives - a set of directives that alter the behaviour of the system
pub type Directives = Vec<DirectiveType>;

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
pub type Entities<'a> = Vec<Entity<'a>>;

/// # Code based Entities
/// This enum specifies wether something is a statement,
/// an assigned variable or a comment (WIP)
#[derive(Debug)]
pub enum Entity<'a> {
    AssignedSequence((Variable<'a>, Sequence<'a>)),
    AssignedAction((Variable<'a>, Action<'a>)),
    MidiBind((Pad, VariableName<'a>)),
    Statement(StatementType<'a>),
    Directive(DirectiveType),
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

/// Type alias for the entire set of values needed to execute the code
pub struct Script<'a> {
    variables: Variables<'a>,
    midi_binds: MidiBinds<'a>,
    directives: Directives,
}

impl Script<'_> {
    /// Checks if midi directive is enabled
    pub fn midi_enabled(self) -> bool {
        self.directives
            .iter()
            .any(|dir| dir == &DirectiveType::Midi)
    }
}

/// Executor for any action that it gets passed
fn execute_entities(action: &Entity, bridge: &HueBridge) {
    use Entity::*;
    match action {
        AssignedAction(((_, _), action)) => interpret_action(action, bridge),
        AssignedSequence(((_, _), sequence)) => interpret_sequence(sequence, bridge),
        _ => panic!("Execute actions doesn't handle Statements or Binds"),
    }
}

/// Interprets one action
fn interpret_action(action: &Action, bridge: &HueBridge) {
    use lighthouse::colors::hex_to_hsl;
    use std::thread::sleep;
    use std::time::Duration;
    use Action::*;
    match action {
        Wait(duration) => sleep(Duration::from_secs(*duration as u64)), // TODO: fix
        Blink((n, pause, color)) => {
            let (h, s, l) = hex_to_hsl(color.to_string()).unwrap();
            for _ in 0..*n {
                bridge
                    .all(state!(hue: h, sat: s, bri: l))
                    .expect("Could not send blink state");
                bridge
                    .all(state!(on: false, hue: h, sat: s, bri: l))
                    .expect("Could not send blink state");
                sleep(Duration::from_secs(*pause as u64));
                bridge
                    .all(state!(on: true, hue: h, sat: s, bri: l))
                    .expect("Could not send blink state");
            }
        }
        Color(color) => {
            let (h, s, l) = hex_to_hsl(color.to_string()).unwrap();
            bridge
                .all(state!(hue: h, sat: s, bri: l))
                .expect("Could not send change color state")
        }
    };
}

/// Interpret sequence
fn interpret_sequence(sequence: &[Action], bridge: &HueBridge) {
    for action in sequence.iter() {
        interpret_action(action, bridge)
    }
}

// TODO: use the assigned actions and sequences to store them rather than unfolding them
// TODO: this needs to panic if an undefined variable is used
// TODO: needs to panic in parsers - sort out proper error reporting
// TODO: make bridge not be there until first statement is encountered - might be a bad idea.
// Get all variables into a hashmap from their their name
// to their entity.
// pub fn runtime(entities: Entities) {
// }
/// # Execute script
/// Returns the variables and the midi mappings
pub fn structure(entities: Entities, bridge: HueBridge) -> Script {
    use Entity::*;
    use StatementType::*;
    let mut midi_binds = HashMap::new();
    let mut variables = HashMap::new();
    let mut directives = Vec::new();
    for entity in entities {
        match entity {
            AssignedAction(((name, _), _)) => {
                variables.insert(name, entity);
            }
            AssignedSequence(((name, _), _)) => {
                variables.insert(name, entity);
            }
            MidiBind((pad, seq)) => {
                midi_binds.insert(pad, seq);
            }
            Statement(Trigger(which)) => execute_entities(&variables[which], &bridge),
            Directive(directive) => directives.push(directive),
        }
    }

    Script {
        variables,
        midi_binds,
        directives,
    }
}
