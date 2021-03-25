use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_till1, take_while_m_n},
    character::complete::multispace0,
    combinator::{map, peek, value as n_value},
    error::context,
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Str(String),
    Boolean(bool),
    Null,
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn normal(i: &str) -> IResult<&str, &str> {
    take_till1(|c: char| c == '\\' || c == '"' || c.is_ascii_control())(i)
}

fn parse_hex(i: &str) -> IResult<&str, &str> {
    context(
        "hex string",
        preceded(
            peek(tag("u")),
            take_while_m_n(5, 5, |c: char| c.is_ascii_hexdigit() || c == 'u'),
        ),
    )(i)
}

fn escapable(i: &str) -> IResult<&str, &str> {
    context(
        "escaped",
        alt((
            tag("\""),
            tag("\\"),
            tag("/"),
            tag("b"),
            tag("f"),
            tag("n"),
            tag("r"),
            tag("t"),
            parse_hex,
        )),
    )(i)
}

fn parse_str(i: &str) -> IResult<&str, &str> {
    escaped(normal, '\\', escapable)(i)
}

fn string(i: &str) -> IResult<&str, &str> {
    context(
        "string",
        alt((tag("\"\""), delimited(tag("\""), parse_str, tag("\"")))),
    )(i)
}

fn boolean(i: &str) -> IResult<&str, bool> {
    let parse_true = n_value(true, tag("true"));
    let parse_false = n_value(false, tag("false"));
    alt((parse_true, parse_false))(i)
}

fn null(i: &str) -> IResult<&str, JsonValue> {
    map(tag("null"), |_| JsonValue::Null)(i)
}

fn value(i: &str) -> IResult<&str, JsonValue> {
    context(
        "value",
        delimited(
            multispace0,
            alt((
                map(object, JsonValue::Object),
                map(array, JsonValue::Array),
                map(string, |s| JsonValue::Str(String::from(s))),
                map(double, JsonValue::Num),
                map(boolean, JsonValue::Boolean),
                null,
            )),
            multispace0,
        ),
    )(i)
}

fn array(i: &str) -> IResult<&str, Vec<JsonValue>> {
    context(
        "array",
        delimited(
            tag("["),
            separated_list0(tag(","), delimited(multispace0, value, multispace0)),
            tag("]"),
        ),
    )(i)
}

fn key(i: &str) -> IResult<&str, &str> {
    delimited(multispace0, string, multispace0)(i)
}

fn object(i: &str) -> IResult<&str, HashMap<String, JsonValue>> {
    context(
        "object",
        delimited(
            tag("{"),
            map(
                separated_list0(
                    tag(","),
                    separated_pair(key, tag(":"), delimited(multispace0, value, multispace0)),
                ),
                |tuple_vec: Vec<(&str, JsonValue)>| {
                    tuple_vec
                        .into_iter()
                        .map(|(k, v)| (String::from(k), v))
                        .collect()
                },
            ),
            tag("}"),
        ),
    )(i)
}

pub fn root(i: &str) -> IResult<&str, JsonValue> {
    delimited(
        multispace0,
        alt((map(object, JsonValue::Object), map(array, JsonValue::Array))),
        multispace0,
    )(i)
}

#[test]
fn testA(){
    let data = "  { \"a\"\t: 42,
    \"b\": [ \"x\", \"y\", 12 ] ,
    \"c\": { \"hello\" : \"world\"
    }
    } ";

    println!(
        "will try to parse valid JSON data:\n\n**********\n{}\n**********\n",
        data
    );
    println!("parsing a valid file:\n{:#?}\n", root(data));

    let data = "
    .a{
            width:10px;
            height:1px;
            border:1px solid #123123;
        }
    ";

    println!(
        "will try to parse valid JSON data:\n\n**********\n{}\n**********\n",
        data
    );
    println!("parsing a valid file:\n{:#?}\n", root(data));



    let data = "  { \"a\"\t: 42,
    \"b\": [ \"x\", \"y\", 12 ] ,
    \"c\": { 1\"hello\" : \"world\"
    }
    } ";
    println!(
        "will try to parse invalid JSON data:\n\n**********\n{}\n**********\n",
        data
    );
    println!(
        "basic errors - `root::<(&str, ErrorKind)>(data)`:\n{:#?}\n",
        root(data)
    );
}