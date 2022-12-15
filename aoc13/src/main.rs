extern crate json;
use json::parse;
use json::JsonValue;
use json::JsonValue::Array;

fn main() {
    println!("Hello, world!");
    let parsed = parse(r#"[1,2,3, [], [4,5]]"#).unwrap();

    let mut arr:Vec<JsonValue> = match parsed {
        Array(l) => l,
        _ => Vec::new(),
    };
    for c in arr.iter() {
        println!("{:?}", c);
        match parsed {
            Array(l) => l,
            Number(n) => Vec::new(),
        };
    }
    // To be continued
}
