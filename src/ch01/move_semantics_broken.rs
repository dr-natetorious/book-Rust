// tag::move_after_process[]
use crate::ch01::parser::{parse_add_order, ParseError};

fn process(_message: crate::ch01::parser::AddOrderMessage) {}

pub fn print_after_move(bytes: &[u8]) -> Result<(), ParseError> {
    let msg = parse_add_order(bytes)?;
    process(msg);
    println!("{:?}", msg);
    Ok(())
}
// end::move_after_process[]