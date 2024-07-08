use nom::character::complete::digit1;
use nom::IResult;

#[test]
fn digit1_test() {
    let s = "63abc";
    let result: IResult<&str, &str> = digit1(&s);
    let (no_used, used) = result.unwrap();
    assert_eq!("63", used);
    assert_eq!("abc", no_used);
}

use super::ast::*;

///定数のパーサ
pub fn constant_val_parser(s: &str) -> IResult<&str, ConstantVal>{
    use std::str::FromStr;
    let (no_used, used) = digit1(s)?;
    let val = FromStr::from_str(used).unwrap();
    Ok((no_used, ConstantVal::new(val)))
}

#[test]
fn consant_val_parser_test() {
    let (_, actual) = constant_val_parser("889").unwrap();
    let expect = ConstantVal::new(889);
    assert_eq!(actual, expect);
}