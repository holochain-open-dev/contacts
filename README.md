# Contacts Zome

Small zome to mange a list of agents that are added/blocked 

This module is designed to be included in other DNAs, assuming as little as possible from those. It is packaged as a holochain zome, and an npm package that offers native Web Components that can be used across browsers and frameworks.

## Documentation

[Hackmd Link](https://hackmd.io/dYCvXSEWRm-StMTg1vpUAg)

## Installation and usage

### Including the zome in your DNA

1. Create a new folder in the `zomes` of the consuming DNA, with the name you want to give to this zome in your DNA.
2. Add a new `Cargo.toml` in that folder. In its content, paste the `Cargo.toml` content from any zome.
3. Change the `name` properties of the `Cargo.toml` file to the name you want to give to this zome in your DNA.
4. Add this zome as a dependency in the `Cargo.toml` file:

```toml
[dependencies]
contacts = {git = "https://github.com/holochain-open-dev/contacts", package = "contacts"}
```

5. Create a `src` folder besides the `Cargo.toml` with this content:

```rust
extern crate contacts;
```

6. Add the zome into your `dna.yaml` file with the name `contacts`.
7. Compile the DNA with the usual `CARGO_TARGET=target cargo build --release --target wasm32-unknown-unknown`.

### Including the UI

TODO

## Developer Setup

See our [developer setup guide](/dev-setup.md).
