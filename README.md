# bd-wasm-module-template

📦 **Template repository** to create **WebAssembly (WASI)** modules compatible with [Bubbledesk](https://bubbledesk.app).

This repo contains the basic structure to develop, test, and build `.wasm` modules to be used inside the **Bubbledesk sandbox** (`bd_sandbox_call`).

---

## Structure

Each module lives in its own dedicated folder:

```
/
 ├── math/            # example module
 │   ├── Cargo.toml
 │   └── src/
 │       ├── functions.rs
 │       ├── function-examples.rs
 │       └── main.rs
 └── module-template/  # empty skeleton ready to copy
     ├── Cargo.toml
     └── src/...
```

- `main.rs`: generic entrypoint, handles stdin/stdout JSON.  
- `functions.rs`: here you register your functions (`fn op_xxx(...)`).  
- `function-examples.rs`: example functions to copy/modify.  

---

## Requirements

- [Rust](https://www.rust-lang.org/) with **WASI** target:
  ```bash
  rustup target add wasm32-wasi
  ```
- [Node.js](https://nodejs.org/) (for the build script in JS).

---

## Build

Each module is built into `.wasm` with:

```bash
npm run build -- <moduleDirName>
```

Example:

```bash
npm run build -- math
```

Result in:

```
dist/math.wasm
```

---

## How it works

### Input/Output

- **Input**: the Bubbledesk sandbox passes JSON via **stdin**:
  Example:
  ```json
  {
    "module_path": "math.wasm",
    "payload": { "fn": "add", "args": [3, 5] },
    "caps": { "timeout_ms": 2000, "memory_mb": 32, "stdout_max_kb": 256 },
    "env": { /* optional */ }
  }
  ```
- **Output**: the module prints JSON via **stdout**:
  ```json
  { "ok": true, "value": 8 }
  ```

### File system

- Each job runs in a sandboxed folder (`/_sandbox/<jobId>`), mapped as `/` inside the module.  
- You can read/write files **only there**.

---

## Workflow for a new module

1. Copy the `module-template` folder with a new name:
    ```bash
    cp -r module-template my-new-module
    ```
2. Edit `Cargo.toml` with the module name and description.
3. Implement your functions in `functions.rs`.
4. Build:
    ```bash
    npm run build -- my-new-module
    ```
5. You can load `dist/my-new-module.wasm` into your Bubbledesk app with:
    ```ts
    await window.Bubbledesk.sandbox.module.add("moduleName");
    ```
    then select `dist/my-new-module.wasm`.

6. You can list loaded modules with:

    ```ts
    await window.Bubbledesk.sandbox.module.list();
    ```
---

## Example call from Bubbledesk

```ts
await window.Bubbledesk.sandbox.call({
  module_path: "math.wasm",
  payload: { fn: "add", args: [3, 5] },
  caps: { timeout_ms: 2000, memory_mb: 32, stdout_max_kb: 256 }
});
// -> { ok: true, value: 42 }
```

---

## Notes

- Keep functions pure and deterministic, without external side effects.
- Do not use network: it is blocked by the sandbox.
- Avoid extra stdout/stderr: only the final JSON output should be printed.
