use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{multispace0, multispace1, none_of};
use nom::combinator::{map, peek};
use nom::error::context;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum CSS {
    Object(HashMap<String, CSS>),
    Value(String),
    ExtendValue(String),
    Import(String),
}
impl CSS {
    pub fn have_import(&self) -> bool {
        let mut r = false;
        match self {
            CSS::Object(d) => {
                for (p, c) in d {
                    match c {
                        CSS::Import(i) => {
                            r = true;
                            break;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        };
        r
    }
}
fn import(i: &str) -> IResult<&str, (String, CSS)> {
    let (i, rsp) = take_while1(|c| c != ':' && c != ';' && c != '}' && c != '{')(i)?;
    let (rsp, _) = tag("@import(")(rsp)?;
    let (_, rsp) = take_while1(|c| c != ')')(rsp)?;
    let (i, _) = tag(";")(i)?;
    Ok((
        i,
        (rsp.trim().to_string(), CSS::Import(rsp.trim().to_string())),
    ))
}
// 有些是;之后是需要消除的
fn end(i: &str) -> IResult<&str, &str> {
    delimited(tag(";"), preceded(multispace0, peek(tag("}"))), multispace0)(i)
}
fn extend(i: &str) -> IResult<&str, (&str, CSS)> {
    let (i, _) = multispace0(i)?;
    let (i, _) = tag("?")(i)?;
    let (i, rsp) = take_while1(|c| c != ':' && c != ';' && c != '}')(i)?;
    // 判断是不是结束
    if let Ok((i, _)) = end(i) {
        return Ok((i, (rsp.trim(), CSS::ExtendValue(rsp.trim().to_string()))));
    };
    Ok((i, (rsp.trim(), CSS::ExtendValue(rsp.trim().to_string()))))
}
fn key(i: &str) -> IResult<&str, &str> {
    let (i, rsp) = take_while1(|c| c != ':' && c != '}' && c != '{')(i)?;
    Ok((i, rsp.trim()))
}
fn value(i: &str) -> IResult<&str, CSS> {
    let (i, rsp) = take_while1(|c| c != ';' && c != '}' && c != '{')(i)?;
    // 判断是不是结束
    if let Ok((i, _)) = end(i) {
        return Ok((i, CSS::Value(rsp.trim().to_string())));
    }
    Ok((i, CSS::Value(rsp.trim().to_string())))
}
fn selector(i: &str) -> IResult<&str, String> {
    let (i, rsp) = take_while1(|c| c != '{' && c != '}')(i)?;
    // 判断是否是key
    Ok((i, rsp.trim().to_string()))
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
            delimited(multispace0, parse, multispace0),
        )),
    )(i)
}
fn parse(i: &str) -> IResult<&str, CSS> {
    context(
        "node",
        delimited(
            multispace0,
            map(
                separated_list1(
                    multispace1,
                    alt((
                        import,
                        separated_pair(selector, tag("{"), terminated(object, tag("}"))),
                    )),
                ),
                |d| CSS::Object(d.into_iter().collect()),
            ),
            multispace0,
        ),
    )(i)
}
#[test]
fn testNewCSS() {
    let data = parse(
        "
        @import(./test1);
        @import(./test2);
        .x a{
            .x{
                .x{
                    .y{
                      width:1px;
                            heigt:1px;
                     ?h-$2

                    }
                    .x{
                      width:1px;
                            heigt:1px;
                    }
                }
            }
        }
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
    // dbg!(data);
    if let Ok((_, data)) = &data {
        dbg!(data.have_import());
    };
    println!("done");
}
