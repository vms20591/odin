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

1. List all brand names - `cargo run list`
2. List all models for a brand - `cargo run list -m -b <brand>`
3. List all models for all brands - `cargo run list -m -a`

**Note:** See `cargo run -- --help` or `cargo run list --help` for more details.

## Sample Output

1. List all brand names - `cargo run list`

```
Found 251 brand(s)!

1. Ocedo - 4 model(s)
2. Akitio - 1 model(s)
3. i.onik - 1 model(s)
4. Telco Electronics - 1 model(s)
5. WAVLINK - 1 model(s)
6. Multilaser - 1 model(s)
7. Pine64 - 2 model(s)
8. ZTE - 4 model(s)
9. Hnet - 1 model(s)
10. Strong - 1 model(s)
11. Turris CZ.NIC - 1 model(s)
12. Creator - 1 model(s)
13. Planex - 11 model(s)
...

Found 251 brand(s)!
```

2. List all models for a brand - `cargo run list -m -b netgear`

```
Brand: Netgear
Found 84 model(s)!

         Model              Version               OpenWrt Version                                  Device Page                                               
         -----              -------               ---------------                                  -----------                                               

1.       D7800              N/A                   https://openwrt.org/releases/19.07.3             https://openwrt.org/toh/netgear/netgear_d7800             
2.       DG834G             v1, v2                https://openwrt.org/releases/10.03.1             https://openwrt.org/toh/netgear/dg834g                    
3.       DG834G             v3                    https://openwrt.org/releases/10.03.1             https://openwrt.org/toh/netgear/dg834g.v3                 
4.       DG834G             v4                    https://openwrt.org/releases/12.09               https://openwrt.org/toh/netgear/dg834g.v4                 
...

Found 84 model(s)!

-----------------------------------------------------------------------------------------------------------------------------------------------------------------------
```