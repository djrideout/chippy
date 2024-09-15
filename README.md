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

# Run interpreter
`cargo run -- <options>`

# Run test suite
`cargo test`

# Build & run (release)
```
cargo build --release
./target/release/chippy.exe <options>
```

# Debugging in VSCode
There is a VSCode launch config for debugging using LLDB in `.vscode/launch.json`.
To use it, modify the args in `.vscode/launch.json` with the options you want from above
and run the configuration `Debug with CHIP-8 ROM` in the `Run and Debug` sidebar tab.
