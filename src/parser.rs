use crate::actions::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::multi::*;
use nom::number::complete::*;
use nom::sequence::*;
use nom::*;

// Basic parsers
fn hello_parser(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::tag("hello")(i)
}
