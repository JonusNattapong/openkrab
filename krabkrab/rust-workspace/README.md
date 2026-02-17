Rust migration workspace scaffold

This folder contains an initial workspace and crate skeletons to begin porting the TypeScript codebase to Rust.

Usage:

1. Run the inventory script to map TypeScript modules:

```bash
python scripts/inventory_ts.py ../src > ts-inventory.txt
```

2. Open the workspace in Cargo-aware editor and start implementing crates under `crates/`.
