use nom::character::complete::digit1
use nom::IResult

#[test]
fn digit1_test() {
    let s = "63abc"
    let result: IResult<&str, &str> = digit1(&s);
    let (no_used, used) = result.unwrap();
    assert_eq!("63", used);
    assert_eq!("abc", no_used);
}