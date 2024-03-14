use std::collections::HashMap;

#[derive(Debug)]
enum JsonData {
    JsonNull,
    JsonInt(i32),
    JsonString(String),
    JsonBool(bool),
    JsonArray(Vec<JsonData>),
    JsonObject(HashMap<String, JsonData>),
}

struct JParser;

impl JParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, mut json_data: String) -> HashMap<String, JsonData> {
        json_data.remove(0);
        json_data.pop();

        let mut json = HashMap::<String, JsonData>::new();

        let mut it = json_data.chars().into_iter();

        let mut key = String::new();
        let mut value: Option<JsonData> = None;
        while let Some(char) = it.next() {
            match char {
                ' ' | '\r' | '\n' | ':' | ',' => {
                    continue;
                }
                '"' if key.is_empty() => {
                    while let Some(next_char) = it.next() {
                        if next_char == '"' {
                            break;
                        }

                        key += &next_char.to_string();
                    }
                }
                '[' | '{' => {
                    let mut value_string = String::new();
                    let end_char = {
                        if char == '[' {
                            ']'
                        } else {
                            value_string += &char.to_string();
                            '}'
                        }
                    };

                    while let Some(next_char) = it.next() {
                        if next_char == end_char {
                            break;
                        }

                        value_string += &next_char.to_string();
                    }

                    if end_char == ']' {
                        value = self.parse_array(&value_string);
                    } else {
                        value_string += &end_char.to_string();
                        value = Some(JsonData::JsonObject(self.parse(value_string)));
                    }
                }
                _ => {
                    let mut value_string = String::new();
                    if char != '"' {
                        value_string += &char.to_string();
                    }

                    while let Some(next_char) = it.next() {
                        if next_char == ','
                            || next_char == '}'
                            || next_char == '\r'
                            || next_char == '\n'
                            || next_char == '"'
                        {
                            break;
                        }

                        value_string += &next_char.to_string();
                    }

                    value = self.try_parse(&value_string);
                }
            }

            if let Some(v) = value {
                json.insert(key.clone(), v);
                key.clear();
                value = None;
            }
        }

        json
    }

    fn parse_string(&self, value: &str) -> Option<JsonData> {
        Some(JsonData::JsonString(String::from(value)))
    }

    fn parse_null(&self, value: &str) -> Option<JsonData> {
        if value == "null" {
            Some(JsonData::JsonNull)
        } else {
            None
        }
    }

    fn parse_bool(&self, value: &str) -> Option<JsonData> {
        if value == "true" {
            Some(JsonData::JsonBool(true))
        } else if value == "false" {
            Some(JsonData::JsonBool(false))
        } else {
            None
        }
    }

    fn parse_int(&self, value: &str) -> Option<JsonData> {
        let int = value.parse();

        if let Ok(i) = int {
            Some(JsonData::JsonInt(i))
        } else {
            None
        }
    }

    fn try_parse(&self, value: &str) -> Option<JsonData> {
        let mut opt: Option<JsonData> = None;

        if opt.is_none() {
            opt = self.parse_null(value);
        }

        if opt.is_none() {
            opt = self.parse_bool(value);
        }

        if opt.is_none() {
            opt = self.parse_int(value);
        }

        if opt.is_none() {
            opt = self.parse_string(value);
        }

        opt
    }

    fn parse_array(&self, value: &str) -> Option<JsonData> {
        let split_values = value.split(",");

        let mut vec = Vec::new();

        for v in split_values {
            let tv = v.trim();
            if let Some(ptv) = self.try_parse(tv) {
                vec.push(ptv);
            }
        }

        Some(JsonData::JsonArray(vec))
    }
}

fn main() {
    let buffer = std::fs::read_to_string("./test.json");

    if let Ok(buf) = buffer {
        let parser = JParser::new();
        let i = parser.parse(buf);

        println!("{:#?}", i);
    }
}