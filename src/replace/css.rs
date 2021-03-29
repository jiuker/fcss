use crate::replace::css::CSS::Object;
use nom::bytes::complete::{is_a, is_not, tag_no_case, take_till, take_while, take_while1};
use nom::bytes::streaming::take_until;
use nom::error::ErrorKind;
use nom::error::VerboseErrorKind::Context;
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
    Value(String),
    VecObject(Vec<CSS>),
}
fn selector_object(i: &str) -> IResult<&str, CSS> {
    // dbg!("selector_object", i.len(), i);
    let (i, rsp) = take_while1(|c| c != '{' && c != ':' && c != ';' && c != '}')(i)?;
    if i.starts_with("{") {
        // dbg!(i.len(), i);
        let mut h = HashMap::new();
        let (i, node) = delimited(
            multispace0,
            alt((
                object,
                delimited(
                    tag("{"),
                    delimited(multispace0, parse_vec, multispace0),
                    tag("}"),
                ),
            )),
            multispace0,
        )(i)?;
        h.insert(rsp.to_string(), node);
        return Ok((i, CSS::Object(h)));
    }
    tag("{")(i)?;
    return Ok(("", CSS::Value("".to_string())));
}
fn object_key_str(i: &str) -> IResult<&str, &str> {
    // dbg!("object_key_str", i.len(), i);
    let (i, rsp) = take_while1(|c| c != '{' && c != ':' && c != ';' && c != '}')(i)?;
    if i.starts_with(":") {
        // dbg!(i);
        return Ok((i, rsp));
    }
    tag(":")(i)?;
    return Ok(("", ""));
}
fn object_value_str(i: &str) -> IResult<&str, &str> {
    // dbg!("object_value_str", i.len(), i);
    let (i, rsp) = take_while1(|c| c != '{' && c != ':' && c != ';' && c != '}')(i)?;
    if i.starts_with(";") {
        // 判断是否结束
        let (_i, _) = tag(";")(i)?;
        let (_i, _) = multispace0(_i)?;
        if _i.starts_with("}") {
            // 这个是结束标志;
            // dbg!(_i);
            return Ok((_i, rsp));
        }
        // dbg!(i);
        return Ok((i, rsp));
    }
    // 有可能没有;就表示结束
    let (i, _) = multispace0(i)?;
    if i.starts_with("}") {
        return Ok((i, rsp.trim()));
    }
    tag(";")(i)?;
    return Ok(("", ""));
}
fn string(i: &str) -> IResult<&str, &str> {
    // dbg!("string", i.len(), i);
    context(
        "string",
        delimited(
            multispace0,
            alt((object_key_str, object_value_str)),
            multispace0,
        ),
    )(i)
}

fn object(i: &str) -> IResult<&str, CSS> {
    // dbg!("object", i.len(), i);
    context(
        "object",
        delimited(
            tag("{"),
            map(
                separated_list0(tag(";"), separated_pair(string, tag(":"), string)),
                |vec| {
                    CSS::Object(
                        vec.into_iter()
                            .map(|c| (c.0.to_string(), CSS::Value(c.1.to_string())))
                            .collect(),
                    )
                },
            ),
            tag("}"),
        ),
    )(i)
}

fn parse(i: &str) -> IResult<&str, CSS> {
    // dbg!("parse", i.len(), i);
    context(
        "value",
        delimited(
            multispace0,
            alt((
                selector_object,
                object,
                map(string, |d| CSS::Value(d.to_string())),
            )),
            multispace0,
        ),
    )(i)
}

fn parse_vec(i: &str) -> IResult<&str, CSS> {
    // dbg!("parse_vec", i.len(), i);
    context(
        "vec",
        map(separated_list0(is_a(".#@{}"), parse), |vec| {
            CSS::VecObject(vec)
        }),
    )(i)
}
#[test]
fn testNewCSS() {
    let data = parse_vec(
        "
        .fade-in-linear-enter-active,.fade-in-linear-leave-active {
	-webkit-transition: opacity 200ms linear;
	transition: opacity 200ms linear
}

.fade-in-linear-enter,.fade-in-linear-leave,.fade-in-linear-leave-active {
	opacity: 0
}

.el-fade-in-linear-enter-active,.el-fade-in-linear-leave-active {
	-webkit-transition: opacity 200ms linear;
	transition: opacity 200ms linear
}

.el-fade-in-linear-enter,.el-fade-in-linear-leave,.el-fade-in-linear-leave-active {
	opacity: 0;
}

.el-fade-in-enter-active,.el-fade-in-leave-active {
	-webkit-transition: all 0.3s cubic-bezier(0.55, 0, 0.1, 1);
	transition: all 0.3s cubic-bezier(0.55, 0, 0.1, 1);
}

.el-fade-in-enter,.el-fade-in-leave-active {
	opacity: 0;
}

.el-zoom-in-center-enter-active,.el-zoom-in-center-leave-active {
	-webkit-transition: all 0.3s cubic-bezier(0.55, 0, 0.1, 1);
	transition: all 0.3s cubic-bezier(0.55, 0, 0.1, 1)
}

.el-zoom-in-center-enter,.el-zoom-in-center-leave-active {
	opacity: 0;
	-webkit-transform: scaleX(0);
	transform: scaleX(0)
}

.el-zoom-in-top-enter-active,.el-zoom-in-top-leave-active {
	opacity: 1;
	-webkit-transform: scaleY(1);
	transform: scaleY(1);
	-webkit-transition: opacity 300ms cubic-bezier(0.23, 1, 0.32, 1),-webkit-transform 300ms cubic-bezier(0.23, 1, 0.32, 1);
	transition: opacity 300ms cubic-bezier(0.23, 1, 0.32, 1),-webkit-transform 300ms cubic-bezier(0.23, 1, 0.32, 1);
	transition: transform 300ms cubic-bezier(0.23, 1, 0.32, 1),opacity 300ms cubic-bezier(0.23, 1, 0.32, 1);
	transition: transform 300ms cubic-bezier(0.23, 1, 0.32, 1),opacity 300ms cubic-bezier(0.23, 1, 0.32, 1),-webkit-transform 300ms cubic-bezier(0.23, 1, 0.32, 1);
	-webkit-transform-origin: center top;
	transform-origin: center top;
}

.el-zoom-in-top-enter,.el-zoom-in-top-leave-active {
	opacity: 0;
	-webkit-transform: scaleY(0);
	transform: scaleY(0);
}

.el-zoom-in-bottom-enter-active,.el-zoom-in-bottom-leave-active {
	opacity: 1;
	-webkit-transform: scaleY(1);
	transform: scaleY(1);
	-webkit-transition: opacity 300ms cubic-bezier(0.23, 1, 0.32, 1),-webkit-transform 300ms cubic-bezier(0.23, 1, 0.32, 1);
	transition: opacity 300ms cubic-bezier(0.23, 1, 0.32, 1),-webkit-transform 300ms cubic-bezier(0.23, 1, 0.32, 1);
	transition: transform 300ms cubic-bezier(0.23, 1, 0.32, 1),opacity 300ms cubic-bezier(0.23, 1, 0.32, 1);
	transition: transform 300ms cubic-bezier(0.23, 1, 0.32, 1),opacity 300ms cubic-bezier(0.23, 1, 0.32, 1),-webkit-transform 300ms cubic-bezier(0.23, 1, 0.32, 1);
	-webkit-transform-origin: center bottom;
	transform-origin: center bottom;
}

.el-zoom-in-bottom-enter,.el-zoom-in-bottom-leave-active {
	opacity: 0;
	-webkit-transform: scaleY(0);
	transform: scaleY(0);
}

.el-zoom-in-left-enter-active,.el-zoom-in-left-leave-active {
	opacity: 1;
	-webkit-transform: scale(1, 1);
	transform: scale(1, 1);
	-webkit-transition: opacity 300ms cubic-bezier(0.23, 1, 0.32, 1),-webkit-transform 300ms cubic-bezier(0.23, 1, 0.32, 1);
	transition: opacity 300ms cubic-bezier(0.23, 1, 0.32, 1),-webkit-transform 300ms cubic-bezier(0.23, 1, 0.32, 1);
	-webkit-transform-origin: top left;
	transform-origin: top left;
}

.el-zoom-in-left-enter,.el-zoom-in-left-leave-active {
	opacity: 0;
	-webkit-transform: scale(0.45, 0.45);
	transform: scale(0.45, 0.45);
}	


.el-zoom-in-left-enter,.el-zoom-in-left-leave-active {
	opacity: 0;
	-webkit-transform: scale(0.45, 0.45);
	transform: scale(0.45, 0.45);
}

.collapse-transition {
	-webkit-transition: 0.3s height ease-in-out, 0.3s padding-top ease-in-out, 0.3s padding-bottom ease-in-out;
	transition: 0.3s height ease-in-out, 0.3s padding-top ease-in-out, 0.3s padding-bottom ease-in-out;
}

.horizontal-collapse-transition {
	-webkit-transition: 0.3s width ease-in-out, 0.3s padding-left ease-in-out, 0.3s padding-right ease-in-out;
	transition: 0.3s width ease-in-out, 0.3s padding-left ease-in-out, 0.3s padding-right ease-in-out;
}

.el-list-enter-active,.el-list-leave-active {
	-webkit-transition: all 1s;
	transition: all 1s;
}

.el-list-enter,.el-list-leave-active {
	opacity: 0;
	-webkit-transform: translateY(-30px);
	transform: translateY(-30px);
}
    ",
    );
    dbg!(data);
    println!("done");
}
