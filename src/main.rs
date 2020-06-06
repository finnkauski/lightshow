use actions::action;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{
    alphanumeric1, digit1, hex_digit1, multispace0, newline, space0, space1,
};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;
use sequences::sequence;

// Type aliases

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
#[derive(Debug)]
pub enum Action<'a> {
    Wait(u16),
    Blink((u16, u16, &'a str)),
    Color(&'a str),
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

/// Helper parsers
mod helpers {
    use super::{digit1, map, multispace0, preceded, space1, tag, terminated, IResult};

    /// Parse a u16 digit
    pub fn u16_digit(i: &str) -> IResult<&str, u16> {
        map(digit1, move |s: &str| s.parse::<u16>().unwrap())(i)
    }

    /// Parse a u16 digit followed by
    pub fn u16_digit_sp(i: &str) -> IResult<&str, u16> {
        terminated(u16_digit, space1)(i)
    }

    /// Makes a parser for a given action name
    pub fn id(which: &'static str) -> impl Fn(&str) -> IResult<&str, &str> {
        move |i| preceded(tag(which), space1)(i)
    }

    /// Generic semicolon and any spaces parser
    pub fn colon(i: &str) -> IResult<&str, &str> {
        terminated(tag(";"), multispace0)(i)
    }
}

/// Operators allowed in the language
mod operators {
    use super::{delimited, space0, tag, IResult};
    // Operators
    /// Parses the assignement operator
    pub fn assign(i: &str) -> IResult<&str, &str> {
        delimited(space0, tag("="), space0)(i)
    }
}

/// Variable name and type parsers
mod variable {
    use super::{
        alphanumeric1, map, pair, preceded, space0, tag, terminated, IResult, Variable,
        VariableType,
    };
    // Variable name and type parsers
    /// Parses a variable name
    fn variable_name(i: &str) -> IResult<&str, &str> {
        terminated(alphanumeric1, tag(":"))(i)
    }

    /// Parse the type of a variable
    fn variable_type(i: &str) -> IResult<&str, VariableType> {
        map(preceded(space0, alphanumeric1), VariableType::from)(i)
    }

    /// Parse both the name and type of the variable
    pub fn variable(i: &str) -> IResult<&str, Variable> {
        pair(variable_name, variable_type)(i)
    }
}

/// Action parsers
/// These parsers are responsible for parsing the actions
mod actions {
    use super::helpers::{colon, id, u16_digit, u16_digit_sp};
    use super::operators::assign;
    use super::variable::variable;
    use super::{alt, hex_digit1, map, pair, preceded, terminated, tuple, Action, Entity, IResult};

    /// Blink action parser - 'blink 1 2 ff00ff' -> Blink((1, 2, "ff00ff"))
    fn blink(i: &str) -> IResult<&str, Action> {
        let blink_params = tuple((u16_digit_sp, u16_digit_sp, hex_digit1));
        map(
            terminated(preceded(id("blink"), blink_params), colon),
            Action::Blink,
        )(i)
    }

    /// Wait action parser - 'wait 1' -> Wait(1)
    fn wait(i: &str) -> IResult<&str, Action> {
        map(
            terminated(preceded(id("wait"), u16_digit), colon),
            Action::Wait,
        )(i)
    }

    /// Color change action parser - 'color ff00ff' -> Color("ff00ff")
    fn color(i: &str) -> IResult<&str, Action> {
        map(
            terminated(preceded(id("color"), hex_digit1), colon),
            Action::Color,
        )(i)
    }

    /// Parse any of the implemented actions
    pub fn generic_action(i: &str) -> IResult<&str, Action> {
        alt((wait, blink, color))(i)
    }

    /// Named action Parser
    pub fn action(i: &str) -> IResult<&str, Entity> {
        map(
            pair(terminated(variable, assign), generic_action),
            Entity::AssignedAction,
        )(i)
    }
}

/// Sequence and bracket parsers
mod sequences {
    use super::actions::generic_action;
    use super::helpers::colon;
    use super::operators::assign;
    use super::variable::variable;
    use super::{
        delimited, many0, map, multispace0, pair, tag, terminated, Entity, IResult, Sequence,
    };

    /// Generic sequence parser - no need for it to be assigned
    fn generic_sequence(i: &str) -> IResult<&str, Sequence> {
        delimited(
            pair(tag("{"), multispace0),
            many0(delimited(multispace0, generic_action, multispace0)),
            pair(tag("}"), colon),
        )(i)
    }

    /// Main sequence parser - needs to be assigned to a variable
    pub fn sequence(i: &str) -> IResult<&str, Entity> {
        map(
            pair(terminated(variable, assign), generic_sequence),
            Entity::AssignedSequence,
        )(i)
    }
}

// Root parser
/// Root parser for the whole documents
fn root(i: &str) -> IResult<&str, Vec<Entity>> {
    many0(terminated(alt((sequence, action)), many0(newline)))(i)
}

fn main() {
    let filename = "/home/art/projects/rust/lightshow/test.lshow";
    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("{:?}", root(&contents[..]));
}
