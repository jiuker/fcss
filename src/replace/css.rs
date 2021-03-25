use nom::bytes::complete::{is_a, tag_no_case, take_till, take_while};
use nom::bytes::streaming::take_until;
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
    Selector(HashMap<String, CSS>),
    Value(String),
    VecSelector(Vec<CSS>),
}

fn selector(i: &str) -> IResult<&str, CSS> {
    let (i, _) = multispace0(i)?;
    let (i, resp) = take_while(|c| c != '{')(i)?;
    let mut s_hash = HashMap::new();
    let (i, v_hash) = object(i)?;
    s_hash.insert(resp.to_string(), CSS::Object(v_hash));
    let (i, _) = multispace0(i)?;
    return Ok((i, CSS::Selector(s_hash)));
}
fn key(i: &str) -> IResult<&str, &str> {
    let (i, _) = multispace0(i)?;
    let (i, resp) = take_while(|c| c != ':')(i)?;
    let (i, _) = multispace0(i)?;
    return Ok((i, resp));
}
fn value(i: &str) -> IResult<&str, CSS> {
    let (i, _) = multispace0(i)?;
    let (i, resp) = take_while(|c| c != ';')(i)?;
    let (i, _) = multispace0(i)?;
    return Ok((i, CSS::Value(resp.to_string())));
}

fn object(i: &str) -> IResult<&str, HashMap<String, CSS>> {
    context(
        "object",
        delimited(
            tag("{"),
            map(
                separated_list0(tag(";"), separated_pair(key, tag(":"), value)),
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
            take_till(|c| c == '}'),
        ),
    )(i)
}
fn parse_key_value(i: &str) -> IResult<&str, CSS> {
    context("selector", selector)(i)
}

pub fn parse_vec_css(i: &str) -> IResult<&str, CSS> {
    context(
        "root",
        delimited(
            multispace0,
            map(separated_list0(tag("}"), parse_key_value), |vec| {
                CSS::VecSelector(vec)
            }),
            multispace0,
        ),
    )(i)
}
#[test]
fn testNewCSS() {
    let data = parse_vec_css(
        "
    .a{
            width:10px;
            height:1px;
            border:1px solid #123123;
        }
 .b{
            width-1:10px;
            height-1:1px;
            border-1:1px solid #123123;
        }
    ",
    );
    dbg!(data);
    println!("done");
}
