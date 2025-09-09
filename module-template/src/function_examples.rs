// This file contains example methods that you can use for reference

use serde_json::{Value, json};

pub fn sum(args: &Value) -> Result<Value, String> {
    let a = args.get("a").and_then(|x| x.as_f64()).unwrap_or(0.0);
    let b = args.get("b").and_then(|x| x.as_f64()).unwrap_or(0.0);
    Ok(json!(a + b))
}

pub fn greet(args: &Value) -> Result<Value, String> {
    let name = args.get("name").and_then(|x| x.as_str()).unwrap_or("world");
    Ok(json!(format!("Hello {}", name)))
}