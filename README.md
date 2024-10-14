# chippy
CHIP-8 interpreter written in Rust. Targets CHIP-8, SUPER-CHIP (Modern), SUPER-CHIP (Legacy), and XO-CHIP.

![Cargo Build & Test](https://github.com/djrideout/chippy/actions/workflows/ci.yml/badge.svg)

# Options
```
  // The CHIP-8 ROM to load
  -i, --input <INPUT>

  // Number of instructions to run per frame, defaults are different depending on the target
  -c, --clock <CLOCK>

  // The platform to target
  -t, --target <TARGET>  [default: super-modern] [possible values: chip, super-modern, super-legacy, xo]
```

# Build requirements
- [Rust/Cargo](https://www.rust-lang.org/tools/install)
- [Node.js if building for web](https://nodejs.org/en)

# Run interpreter (native)
`cargo run -- <options>`

# Run interpreter (web)
```
cd web/view
npm install
npm run dev:full
```

To rebuild wasm only:
`npm run build:wasm`

# Build & run (release, native)
```
cargo build --release
./target/release/chippy.exe <options>
```

# Build & run (release, web)
```
cd web/view
npm install
npm run build:full
npm run preview
```

# Run test suite
`cargo test`

# Debugging in VSCode
There is a VSCode launch config for debugging using LLDB in `.vscode/launch.json`.
To use it, modify the args in `.vscode/launch.json` with the options you want from above
and run the configuration `Debug with CHIP-8 ROM` in the `Run and Debug` sidebar tab.
