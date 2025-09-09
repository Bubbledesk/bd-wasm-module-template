// build.js
// Usage: npm run build -- <dirName>

const fs = require("fs");
const path = require("path");
const { spawn } = require("child_process");

const args = process.argv.slice(2);
if (args.length < 1) {
  console.error("❌ Please provide a directory name (e.g. math)");
  process.exit(1);
}

const dir = args[0];

// Run cargo build in that directory
const child = spawn("cargo", ["build", "--release", "--target", "wasm32-wasip1"], {
  stdio: "inherit",
  cwd: `${__dirname}/${dir}`, // set working directory
});

child.on("exit", (code) => {
  if (code !== 0) {
    console.error(`❌ Cargo build failed with code ${code}`);
    process.exit(code);
  } else {
    console.log(`✅ Built ${dir}/target/wasm32-wasip1/release/*.wasm`);
    const releaseDir = path.join(__dirname, dir, "target", "wasm32-wasip1", "release");
    const files = fs.readdirSync(releaseDir).filter(f => f.endsWith(".wasm"));
    if (files.length > 0) {
      const wasmFile = files[0];
      const src = path.join(releaseDir, wasmFile);
      const dest = path.join(__dirname, dir, wasmFile);
      fs.copyFileSync(src, dest);
      console.log(`📦 Copied ${wasmFile} to ${dir}/`);
    } else {
      console.warn("⚠️ No .wasm file found in release directory");
    }
  }
});