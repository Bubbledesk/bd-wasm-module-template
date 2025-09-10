
use serde_json::{Value, json};
use crate::function_examples::*; // optional, just for reference/examples
use crate::functions::*;

/// Dispatcher: map "fn" field to functions name.
/// Add your custom functions in this match.
pub fn dispatch(op: &str, args: &Value) -> Result<Value, String> {
    match op {
        "ping" => ping(args),

        "sum" => eg_sum(args),       // this is from function_examples.rs, you can remove this
        "greet" => eg_greet(args),   // this is from function_examples.rs, you can remove this

        /* ============================

            ADD HERE YOUR FUNCTIONS

            e.g. "yourFunctionName" => your_function(args),

        ============================ */

        _ => Err(format!("unknown function: {}", op)),
    }
}