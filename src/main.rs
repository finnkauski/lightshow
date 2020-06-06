use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::character::*;
use nom::combinator::*;
use nom::multi::*;
use nom::number::complete::*;
use nom::sequence::*;
use nom::*;

type Variable<'a> = (&'a str, Type);
type Sequence<'a> = Vec<Action<'a>>;
type AssignedSequence<'a> = (Variable<'a>, Vec<Action<'a>>);

#[derive(Debug)]
pub enum Action<'a> {
    Wait(u16),
    Blink((u16, u16, &'a str)),
    Color(&'a str),
}

#[derive(Debug)]
pub enum Type {
    Sequence,
}

impl From<&str> for Type {
    fn from(i: &str) -> Type {
        match i {
            "seq" => Type::Sequence,
            _ => panic!("Unknown type"),
        }
    }
}

// Helper parsers
/// Parse a u16 digit
fn u16_digit(i: &str) -> IResult<&str, u16> {
    map(digit1, move |s: &str| s.parse::<u16>().unwrap())(i)
}

/// Parse a u16 digit followed by
fn u16_digit_sp(i: &str) -> IResult<&str, u16> {
    terminated(u16_digit, space1)(i)
}

/// makes a parser for a given action name
fn id(which: &'static str) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i| preceded(tag(which), multispace1)(i)
}

// Operators
/// Parses the assignement operator
fn assign(i: &str) -> IResult<&str, &str> {
    delimited(space0, tag("="), space0)(i)
}

// Sequence and bracket parsers
/// Seperator for the sequence
fn seq_sep(i: &str) -> IResult<&str, (&str, Option<char>)> {
    terminated(preceded(space0, pair(tag(";"), opt(newline))), space0)(i)
}

/// Generic sequence parser - no need for it to be assigned
fn generic_sequence(i: &str) -> IResult<&str, Sequence> {
    delimited(
        pair(char('{'), newline),
        terminated(
            preceded(space0, separated_list(seq_sep, parse_action)),
            seq_sep,
        ),
        preceded(space0, pair(tag("};"), opt(newline))),
    )(i)
}

fn sequence(i: &str) -> IResult<&str, AssignedSequence> {
    pair(terminated(variable, assign), generic_sequence)(i)
}

// Variable name and type parsers
/// Parses a variable name
fn variable_name(i: &str) -> IResult<&str, &str> {
    terminated(alphanumeric1, tag(":"))(i)
}

fn variable_type(i: &str) -> IResult<&str, Type> {
    map(preceded(space0, alphanumeric1), Type::from)(i)
}

fn variable(i: &str) -> IResult<&str, Variable> {
    pair(variable_name, variable_type)(i)
}

// Action Parsers
/// Blink action parser - 'blink 1 2 ff00ff' -> Blink((1, 2, "ff00ff"))
pub fn blink(i: &str) -> IResult<&str, Action> {
    let blink_params = tuple((u16_digit_sp, u16_digit_sp, hex_digit1));
    map(preceded(id("blink"), blink_params), Action::Blink)(i)
}

/// Wait action parser - 'wait 1' -> Wait(1)
pub fn wait(i: &str) -> IResult<&str, Action> {
    map(preceded(id("wait"), u16_digit), Action::Wait)(i)
}

/// Color change action parser - 'color ff00ff' -> Color("ff00ff")
pub fn color(i: &str) -> IResult<&str, Action> {
    map(preceded(id("color"), hex_digit1), Action::Color)(i)
}

/// Parse any of the implemented actions
pub fn parse_action(i: &str) -> IResult<&str, Action> {
    alt((wait, blink, color))(i)
}

fn main() {
    let filename = "/home/art/projects/rust/lightshow/test.lshow";
    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("{:?}", many0(sequence)(&contents[..]));

    //     println!(
    //         "{:?}",
    //         sequence("a: seq = { color ff0000  ;\n  blink 3 2 ff0000;}")
    //     );
    // //     println!("{:?}", wait("wait 1"));
    //     println!("{:?}", blink("blinkr goodbye hello again"));
    //     println!("{:?}", blink("blink 1 2 ff0000"));
    //     println!("{:?}", blink("blink 1 2 ff0000"));
    //     println!("{:?}", sequence("{    blink 3 2 ff0000;}"));
    //     println!("{:?}", variable_name("a: seq"));
    //     println!("{:?}", variable_type(" seq"));
}
