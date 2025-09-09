# bd-wasm-module-template

📦 **Template repository** per creare moduli **WebAssembly (WASI)** compatibili con [Bubbledesk](https://bubbledesk.app).

Questa repo contiene la struttura di base per sviluppare, testare e buildare moduli `.wasm` da usare dentro il **sandbox di Bubbledesk** (`bd_sandbox_call`).

---

## Struttura

Ogni modulo vive in una sua cartella dentro `modules/`:

```
modules/
 ├── math/          # esempio di modulo
 │   ├── Cargo.toml
 │   └── src/
 │       ├── main.rs
 │       ├── methods.rs
 │       └── examples.rs
 └── template/      # scheletro vuoto pronto da copiare
     ├── Cargo.toml
     └── src/...
```

- `main.rs`: entrypoint generico, gestisce stdin/stdout JSON.  
- `methods.rs`: qui registri le tue funzioni (`fn op_xxx(...)`).  
- `examples.rs`: funzioni d’esempio da copiare/adattare.  

---

## Requisiti

- [Rust](https://www.rust-lang.org/) con target **WASI**:
  ```bash
  rustup target add wasm32-wasi
  ```
- [Node.js](https://nodejs.org/) (per lo script di build in JS).

---

## Build

Ogni modulo viene buildato in `.wasm` con:

```bash
npm run build -- <moduleDirName>
```

Esempio:

```bash
npm run build -- math
```

Risultato in:

```
dist/math.wasm
```

---

## Come funziona

### Input/Output

- **Input**: il sandbox di Bubbledesk passa JSON via **stdin**:
  ```json
  { "fn": "sum", "args": [3, 5] }
  ```
- **Output**: il modulo stampa JSON via **stdout**:
  ```json
  { "ok": true, "value": 8 }
  ```

### File system

- Ogni job gira in una cartella sandboxata (`/_sandbox/<jobId>`), mappata come `/` dentro il modulo.  
- Puoi leggere/scrivere file **solo lì**.

---

## Workflow per un nuovo modulo

1. Copia la cartella `modules/template` con un nuovo nome:
   ```bash
   cp -r modules/template modules/my-new-module
   ```
2. Modifica `Cargo.toml` con nome e descrizione del modulo.
3. Implementa le tue funzioni in `methods.rs`.
4. Builda:
   ```bash
   npm run build -- my-new-module
   ```
5. Copia `dist/my-new-module.wasm` in `_external_modules/` di Bubbledesk.

---

## Esempio di chiamata da Bubbledesk

```ts
// Comments in English
await window.Bubbledesk.sandbox.call("math.wasm", {
  fn: "sum",
  args: [10, 32]
});
// -> { ok: true, value: 42 }
```

---

## Note

- Mantieni le funzioni pure e deterministic, senza side-effect esterni.
- Non usare network: è bloccato dal sandbox.
- Evita stdout/stderr extra: deve esserci solo l’output JSON finale.

---

## Licenza

MIT
