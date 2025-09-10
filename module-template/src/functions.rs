
use serde_json::{Value, json};

/* ============================
   YOUR FUNCTIONS (BEGIN)
   - Define your functions here as `fn function_name(args: &Value) -> Result<Value, String>`.
   - Functions should:
       * Read inputs from `args` (array or object as you prefer).
       * Return `Ok(Value)` on success, or `Err(String)` on error.
   - If you need filesystem, use paths under "/" (preopened to the sandbox work dir).
   - Avoid stdout/stderr prints except the final JSON response.
   ============================ */

// ADD YOUR FUNCTIONS HERE

pub fn ping(_args: &Value) -> Result<Value, String> {
    Ok(json!({"pong": true}))
}

/* ============================
   YOUR FUNCTIONS (END)
   TODO: You must add your functions to dispatcher.rs to expose them.
   ============================ */
