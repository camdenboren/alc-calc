# alc-calc

![Static Badge](https://img.shields.io/badge/Platforms-Linux,_macOS-forestgreen?style=for-the-badge)
![Static Badge](https://img.shields.io/badge/Powered_by_Nix-grey?logo=nixOS&logoColor=white&logoSize=auto&style=for-the-badge)
[![built with garnix](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgarnix.io%2Fapi%2Fbadges%2Fcamdenboren%2Falc-calc%3Fbranch%3Dmain&style=for-the-badge&color=grey&labelColor=grey)](https://garnix.io/repo/camdenboren/alc-calc)

This is a VERY early stage Rust rewrite of my first side project: [Alcohol_Weight_Calculator]

## Motivation

Weight-based measurement is growing in popularity for many in the kitchen, so why not apply this improved accuracy and consistency to making drinks?

The original project is a dead-simple Python CLI for quickly calculating the weight of ingredients in an alcoholic beverage. It has served me well, but it has notable limitations

- No input validation (shameful, I know)
- Inability to add non-alcoholic ingredients
- An emphasis on broadly categorizing ingredients handicaps precision when an ingredient's actual density is already known
- No GUI

The first three have relatively simple solutions that I could implement in a weekend, but the thought of retrofitting yet another Python GUI failed to excite me

Enter [GPUI], the UI Framework from the Zed team

- Tailwind's ease-of-use
- Rust's speed and safety
- First-class hardware acceleration

Rust rewrite with an undocumented framework it is :moyai:

## Structure

### Source

`src` contains two crates

- The binary crate (`main`) serves only as an entry point into the library crate, which contains the UI and calculation logic

- `ui` contains the main window, with individual views and components delegated to other modules like `titlebar`

### Build System

Cargo is the underlying build system, _but_ Nix will be the official 'meta' build system for the foreseeable future

For the uninitiated, Nix reproducibly supplies all project dependencies (including rust, Darwin SDKs, custom scripts, etc.) without the need for containers. From a practical standpoint, this means any Linux or macOS user can _reliably_ run this project with a single command

```shell
nix run github:camdenboren/alc-calc
```

I'll need to get my head wrapped around a few things before I accept contributions, but the `build` and `format` scripts are very useful for any contributors (and are required before PRs)

You can access the development environment (including these scripts) with

```shell
nix develop github:camdenboren/alc-calc
```

## License

[GPLv3]

[Alcohol_Weight_Calculator]: https://github.com/camdenboren/Alcohol_Weight_Calculator
[GPUI]: https://www.gpui.rs/
[GPLv3]: COPYING
