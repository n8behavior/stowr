# stowr-cli

A simple command-line interface for the [stowr](../../README.md) asset management workspace.

## Building

From the workspace root you can build just this crate:

```bash
cargo build -p stowr-cli --release
```

This will place the compiled binary at `target/release/stowr-cli`.

## Running

During development it's convenient to run the CLI through Cargo. Pass any
arguments after `--`:

```bash
cargo run -p stowr-cli -- --name sample
```

After building (or installing via `cargo install --path crates/cli`), you can
invoke the binary directly:

```bash
./target/release/stowr-cli --name sample
```

```
Asset [sample] not found...yet!
```

Further functionality will be added as the project evolves.
