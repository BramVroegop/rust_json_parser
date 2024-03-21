mod parser;

fn main() {
    let buffer = std::fs::read_to_string("./test.json");

    if let Ok(buf) = buffer {
        let parser = parser::JParser::new();

        if let Some(value) = parser.parse(buf) {
            println!("{:#?}", value);
        }
    }
}
