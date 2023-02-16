# Sticky Note

Minimalist command-line program to record 'sticky notes'. Basically a wrapper around [ex0dus-0x](https://github.com/ex0dus-0x)'s [microkv](https://github.com/ex0dus-0x/microkv) crate, differing from the provided `microkv-cli` with simplified arguments/flags (no passwords, and assumed DB name).

# Install

Currently not published to crates.io. Clone and enter the repo, and install locally with:

```
cargo install --path ./
```

This installs the project as a binary named `sn`.

# Usage

The following commands are supported:
* `sn list`:  Lists all sticky note names and a brief preview of their contents.
* `sn read <name>`:  Prints the full sticky note with the given name.
* `sn write <name> <text>`:  Writes a new sticky note with the given name - no-ops if a note with the given the name already exists.
* `sn edit <name> <text>`:  Edits the sticky note with the given name - writes a new one if a note with the given the name couldn't be found.
* `sn delete <name>`:  Deletes the note with the given name.
