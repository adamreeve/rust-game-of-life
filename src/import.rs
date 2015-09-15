use std::fs::File;
use std::io::Read;
use std::path::Path;
use combine::{
    State, ParseResult, ParserExt,
    digit, many, many1, optional, parser, satisfy,
    sep_end_by, skip_many, skip_many1, space, token
};
use combine::primitives::{Parser, Stream};

use life;

pub fn load_file(input_path: &str) -> Result<life::WorldState, String> {
    let path = Path::new(input_path);
    let path_display = path.display();

    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(err) => return Err(format!("Error opening {}: {}", path_display, err))
    };
    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Ok(_) => match parser(parse_life).parse(&file_content[..]).map(|(output, _)| output) {
            Ok(world) => Ok(world),
            Err(err) => Err(format!("Error parsing {}: {}", path_display, err)),
        },
        Err(err) => return Err(format!("Error reading {}: {}", path_display, err))
    }
}

fn parse_life<I>(input: State<I>) -> ParseResult<life::WorldState, I>
where I: Stream<Item=char> {
    (skip_many(parser(comment)), sep_end_by(parser(position), parser(line_break))).map(|(_, positions)| positions).parse_state(input)
}

fn comment<I>(input: State<I>) -> ParseResult<String, I>
where I: Stream<Item=char> {
    (token('#'), many(satisfy(|c| c != '\n')), token('\n')).map(|(_, comment, _)| comment).parse_state(input)
}

fn integer<I>(input: State<I>) -> ParseResult<i32, I>
where I: Stream<Item=char> {
    let parse_uint = many1(digit()).map(|string: String| string.parse::<i32>().unwrap());
    (optional(token('-')), skip_many(space()), parse_uint).map(|(neg, _, uint_val)| match neg {
        Some(_) => -1 * uint_val,
        None => uint_val
    }).parse_state(input)
}

fn position<I>(input: State<I>) -> ParseResult<life::CellPosition, I>
where I: Stream<Item=char> {
    (parser(integer), skip_many1(space()), parser(integer)).map(
        |(x, _, y)| life::CellPosition {
            x: x + 10,
            y: y + 10
        }).parse_state(input)
}

fn line_break<I>(input: State<I>) -> ParseResult<(), I>
where I: Stream<Item=char> {
    (skip_many(token(' ').or(token('\t'))), token('\n')).map(|_| ()).parse_state(input)
}
