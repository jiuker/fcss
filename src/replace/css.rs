use nom::bytes::complete::take_while;
use nom::multi::separated_list1;
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
pub enum CSS {
    Object(HashMap<String, CSS>),
    Selector(String),
    Value(String),
}

fn selector(i: &str) -> IResult<&str, CSS> {
    let (i, _) = multispace0(i)?;
    let (i, resp) = take_while(|c| c != '{')(i)?;
    return Ok((i, CSS::Selector(resp.to_string())));
}
fn key(i: &str) -> IResult<&str, &str> {
    dbg!(i);
    let (i, _) = multispace0(i)?;
    dbg!(i);
    let (i, resp) = take_while(|c| c != ':')(i)?;
    dbg!(i);
    return Ok((i, resp));
}
fn value(i: &str) -> IResult<&str, CSS> {
    dbg!(i);
    let (i, _) = multispace0(i)?;
    dbg!(i);
    let (i, resp) = take_while(|c| c != ';')(i)?;
    dbg!(i);
    return Ok((i, CSS::Value(resp.to_string())));
}
fn object(i: &str) -> IResult<&str, HashMap<String, CSS>> {
    println!("{}", i);
    context(
        "object",
        delimited(
            tag("{"),
            map(
                separated_list0(
                    tag(";"),
                    separated_pair(key, tag(":"), delimited(multispace0, value, multispace0)),
                ),
                |tuple_vec| {
                    tuple_vec
                        .into_iter()
                        .map(|(k, v)| {
                            println!("{}{:?}", k, v);
                            (k.to_string(), v)
                        })
                        .collect()
                },
            ),
            delimited(tag(";"), multispace0, tag("")),
        ),
    )(i)
}
fn parse_key_value(i: &str) -> IResult<&str, HashMap<String, CSS>> {
    context("selector", delimited(selector, object, multispace0))(i)
}
pub fn root(i: &str) -> IResult<&str, CSS> {
    context(
        "root",
        delimited(multispace0, map(parse_key_value, CSS::Object), multispace0),
    )(i)
}

#[test]
fn testNewCSS() {
    let data = root(
        "
    .a{
            width:10px;
            height:1px;
            border:1px solid #123123;
        }
    ",
    );
    dbg!(data);
    println!("done");
}
