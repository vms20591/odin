# ODIN - OpenWrt Device Information

This is a simple CLI program that's meant to get some details on supported devices from OpenWrt's [supported devices](https://openwrt.org/toh/start) page.

## Build

### Clone this repo

```
git clone <this-repo>
```

### Build this repo

```
cd <this-repo>
cargo build
```

## Usage

1. List all brands names - `cargo run list`
2. List all models for a brand - `cargo run list -m -b netgear`
3. List all models for all brands - `cargo run list -m -a`

**Note:** See `cargo run -- --help` or `cargo run list --help` for more details.