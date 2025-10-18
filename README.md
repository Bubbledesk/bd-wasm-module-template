# bd-wasm-module-template

📦 **Template repository** to create **WebAssembly (WASI)** modules compatible with [Bubbledesk](https://bubbledesk.app).

This repo contains the basic structure to develop, test, and build `.wasm` modules to be used inside the **Bubbledesk sandbox** (`bd_sandbox_call`).


Bubbledesk’s **Plugins** provides a secure, sandboxed and flexible runtime for executing [WebAssembly (WASM)](https://webassembly.org/) plugins within the desktop wrapper.  
This allows developers to extend app capabilities and Bubbledesk to release features without forcing new builds.

## Overview

Each worker runs in its own hidden webview window (`bd-worker`) and executes WASM modules inside a Web Worker environment.
The runtime isolates every execution, providing:
- **File system sandboxing**
- **Automatic timeout and queue limits**
- **JSON-based communication**
- **Cross-platform consistency (macOS, Windows, Linux)**

Workers are managed via the JavaScript bridge at `window.Bubbledesk.worker`.


## Architecture

The system is composed of three layers:

| Layer | Language | Responsibility |
|--------|-----------|----------------|
| **Frontend** | JavaScript or TypeScript | Provides the API under `window.Bubbledesk.worker` |
| **Backend Host** | Rust + Tauri | Manages worker lifecycle, WASM validation, job queueing, and communication |
| **Execution Layer (Web Worker)** | JavaScript + [WASM](https://webassembly.org/) | Executes the loaded WebAssembly module and streams stdout/stderr |

All communication between host and worker is event-driven via the Tauri IPC system.

## Input/Output Standard

The communication protocol is **line-delimited JSON**:

### Input (from host → plugin)

The Bubbledesk sandbox passes JSON via **stdin**:

```json
{ "fn": "add", "args": [3, 5] }
```
> *this is an example*

### Output (from plugin → host)

The plugin must write **a single line** of JSON to **stdout**, terminated by `\n`.

#### Success:
```json
{"ok": true, "value": 8}
```
> *this is an example*

#### Error:
```json
{"ok": false, "error": "division by zero"}
```
> *this is an example*

This format is mandatory — the worker will reject malformed or multiline JSON.

## Security and Sandbox

- Each plugin executes in its own **isolated environment**.
    > Each job runs in a sandboxed folder (`/_sandbox/<jobId>`), mapped as `/` inside the module. Here (and here only) you can read/write files with the plugin.
- **No network access** or external FS access is allowed.
- Max 64 concurrent pending jobs.
- Input limited to 1 MB per call.
- WASM modules must start with the standard `\0asm` magic bytes.



## WASM Plugin Structure

Each plugin is an independent **Rust project** compiled for the `wasm32-wasip1` target and executed as a WASI-compatible process.

Example structure:
```
math/
 ├─dist/
 |   └─ math.wasm
 ├─ src/
 │   ├─ main.rs
 │   ├─ dispatcher.rs
 │   └─ functions.rs
 ├─ Cargo.toml
 └─ package.json
```

Plugins communicate with the host via **stdin/stdout**, using a strict JSON protocol.

## Requirements

- [Rust](https://www.rust-lang.org/) with **WASI** target:

  ```bash
  rustup target add wasm32-wasi
  ```
- [Node.js](https://nodejs.org/) (for the build JS script `build.js`).

## Build

Each module is built into `.wasm` with:

```bash
npm run build -- <moduleDirName>
```

Example:

```bash
npm run build -- math
```

Result in `dist/` folder, example:

```
dist/math.wasm
```


## Directory Layout

| Directory | Purpose |
|------------|----------|
| `/_external_modules/` | Holds installed WASM modules |
| `/_sandbox/<jobId>` | Temporary runtime area, wiped at startup |

## Develop a Plugin

1. Fork the [`bd-wasm-module-template`](https://github.com/Bubbledesk/bd-wasm-module-template) repo from Bubbledesk's GitHub:
    ```url
    https://github.com/Bubbledesk/bd-wasm-module-template
    ```

1. Copy the `module-template` folder with a new name:
    ```bash
    cp -r module-template my-new-module
    ```
2. Edit `Cargo.toml` with the module name and description.
3. Implement your functions in `functions.rs` and register them in `dispatcher.rs`.
4. Build:
    ```bash
    npm run build -- my-new-module
    ```
5. You can load `dist/my-new-module.wasm` into your Bubbledesk app with:
    ```ts
    await window.Bubbledesk.worker.modules.add("moduleName");
    ```
    then select `dist/my-new-module.wasm`.

6. You can list loaded modules with:

    ```ts
    await window.Bubbledesk.worker.modules.list();
    ```
7. You can call it like this:
    ```js
    const res = await window.Bubbledesk.worker.call("moduleName.wasm", {
        fn: "function_name",
        args: [...]
    });

    console.log(JSON.parse(res).value);
    ```
    
---

### Notes

- Workers are managed via the JavaScript bridge at `window.Bubbledesk.worker`.
- Keep functions pure and deterministic, without external side effects.
- Do not use network: it is blocked by the sandbox.
- Avoid extra stdout/stderr: only the final JSON output should be printed.
- All modules must follow the **stdin/stdout JSON protocol** strictly. Multi-line, non-JSON, or binary outputs will be discarded.
- Avoid infinite loops or blocking operations inside your WASM module — they will trigger a timeout and the worker will be restarted.
