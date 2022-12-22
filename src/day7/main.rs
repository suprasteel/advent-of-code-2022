use nom::sequence::preceded;

#[macro_use]
extern crate derive_new;

#[cfg(test)]
mod test;

mod fs;

fn main() {
    // TODO:
    // - parsing with nom
    // - handmade tree structure fun fun and practise
}

/*
fn hello_parser(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::tag("hello")(i)
}
*/

/*
fn parse_dollar(i: &str) -> nom::IResult<&str, &str> {
    preceded(tag("$ "))(i)
}
*/
