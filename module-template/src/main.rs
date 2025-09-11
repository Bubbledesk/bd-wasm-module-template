
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Import your dispatcher
mod dispatcher;
mod functions;
mod function_examples; // optional, just for reference/examples
use dispatcher::dispatch;

/// Request shape: host sends JSON to stdin.
#[derive(Debug, Deserialize)]
struct Request {
    #[serde(rename = "fn")]
    op: String,
    #[serde(default)]
    args: Value,
}

/// Response envelope
#[derive(Debug, Serialize)]
#[serde(untagged)]
enum Response {
    Ok  { ok: bool, value: Value },
    Err { ok: bool, error: String },
}

/// Print JSON response
fn print_json(v: &Response) {
    let s = serde_json::to_string(v).unwrap_or_else(|_| "{\"ok\":false,\"error\":\"serialize\"}".to_string());
    let _ = std::io::stdout().write_all(s.as_bytes());
    let _ = std::io::stdout().write_all(b"\n");
    let _ = std::io::stdout().flush();
}

fn handle_stdin_once() -> Result<Response, String> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).map_err(|e| format!("stdin: {}", e))?;
    if buf.trim().is_empty() {
        return Err("empty stdin".into());
    }
    let req: Request = serde_json::from_str(&buf).map_err(|e| format!("json: {}", e))?;
    match dispatch(&req.op, &req.args) {
        Ok(value) => Ok(Response::Ok { ok: true, value }),
        Err(error) => Ok(Response::Err { ok: false, error }),
    }
}

fn fallback_usage() -> Response {
    Response::Err {
        ok: false,
        error: "usage: send JSON {\"fn\":<string>, \"args\":<json>}".into(),
    }
}

fn main() {
    // Debug marker: if this file appears, the module has entered _start
    let _ = std::fs::write("/__entered_start", b"1");
    match handle_stdin_once() {
        Ok(resp) => print_json(&resp),
        Err(_) => print_json(&fallback_usage()),
    }
}