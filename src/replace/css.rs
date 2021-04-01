use crate::replace::css::CSS::Object;
use nom::bytes::complete::{is_a, is_not, tag_no_case, take, take_till, take_while, take_while1};
use nom::bytes::streaming::take_until;
use nom::character::complete::{multispace1, none_of, satisfy};
use nom::character::is_alphabetic;
use nom::error::ErrorKind;
use nom::error::VerboseErrorKind::Context;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_till1, take_while_m_n},
    character::complete::multispace0,
    combinator::{map, peek, value as n_value},
    error::context,
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum CSS {
    Object(HashMap<String, CSS>),
    Value(String),
    VecObject(Vec<CSS>),
    ExtendValue(String),
}
fn extend(i: &str) -> IResult<&str, (&str, CSS)> {
    let (i, _) = multispace0(i)?;
    let (i, _) = tag("?")(i)?;
    let (i, rsp) = take_while1(|c| c != ':' && c != ';' && c != '}')(i)?;
    // 判断是不是结束
    if i.starts_with(";") {
        let (i_, _) = tag(";")(i)?;
        let (i_, _) = multispace0(i_)?;
        if i_.starts_with("}") {
            return Ok((i_, (rsp.trim(), CSS::ExtendValue(rsp.trim().to_string()))));
        }
    }
    Ok((i, (rsp.trim(), CSS::ExtendValue(rsp.trim().to_string()))))
}
fn key(i: &str) -> IResult<&str, &str> {
    let (i, rsp) = take_while1(|c| c != ':' && c != '}' && c != '{')(i)?;
    // 判断是否是key
    none_of("{}")(rsp.trim())?;
    Ok((i, rsp.trim()))
}
fn value(i: &str) -> IResult<&str, CSS> {
    let (i, rsp) = take_while1(|c| c != ';' && c != '}' && c != '{')(i)?;
    // 判断是不是结束
    if i.starts_with(";") {
        let (i_, _) = tag(";")(i)?;
        let (i_, _) = multispace0(i_)?;
        if i_.starts_with("}") {
            return Ok((i_, CSS::Value(rsp.trim().to_string())));
        }
    }
    Ok((i, CSS::Value(rsp.trim().to_string())))
}
fn selector(i: &str) -> IResult<&str, &str> {
    let (i, _) = multispace0(i)?;
    let (i, rsp) = take_while1(|c| c != '{' && c != '}')(i)?;
    // 判断是否是key
    none_of("{}")(rsp.trim())?;
    Ok((i, rsp.trim()))
}
fn object(i: &str) -> IResult<&str, CSS> {
    context(
        "object",
        alt((
            delimited(
                multispace0,
                map(
                    separated_list1(
                        tag(";"),
                        alt((extend, separated_pair(key, tag(":"), value))),
                    ),
                    |d| CSS::Object(d.into_iter().map(|(k, v)| (k.to_string(), v)).collect()),
                ),
                multispace0,
            ),
            delimited(multispace0, parse_vec, multispace0),
        )),
    )(i)
}
fn parse(i: &str) -> IResult<&str, CSS> {
    context(
        "node",
        (map(
            separated_pair(selector, tag("{"), terminated(object, tag("}"))),
            |(k, v)| {
                let mut h = HashMap::new();
                h.insert(k.to_string(), v);
                CSS::Object(h)
            },
        )),
    )(i)
}
fn parse_vec(i: &str) -> IResult<&str, CSS> {
    context(
        "vec",
        delimited(
            multispace0,
            map(separated_list1(multispace1, parse), CSS::VecObject),
            multispace0,
        ),
    )(i)
}
#[test]
fn testNewCSS() {
    let data = parse_vec(
        "
        .a{
                            width:1px;
                            heigt:1px;

        }
 .b{
                            width:1px;
                            heigt:1px

        }
 .c{
                            heigt:1px;

        }
        .d1{
            .a1{
                ?w-$1;
                width:1px;
                 ?h-$2
            }
            .b1{
                ?w-$1;
                 ?h-$2
            }
            .c1{
                 .a1{
                    ?w-$1;
                     ?h-$2
                }
            }
        }
        .e2{
            .a2{
                ?w-$1;
                 ?h-$2
            }
            .b2{
                ?w-$1;
                 ?h-$2
            }
        }
    ",
    );
    /*

    */
    dbg!(data);
    println!("done");
}
