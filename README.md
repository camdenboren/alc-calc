<h1 align="center">
    <img src="./img/brand/app-icon@2x.png" width="100" alt="Logo"><br/>
    alc-calc
</h1>

<div align="center">
    <p>
        alc-calc is a GUI calculator for measuring alcoholic beverages by weight, not volume
    </p>

![Static Badge](https://img.shields.io/badge/Platforms-Linux,_macOS,_Windows-forestgreen?style=for-the-badge)
[![built with garnix](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgarnix.io%2Fapi%2Fbadges%2Fcamdenboren%2Falc-calc%3Fbranch%3Dmain&style=for-the-badge&color=grey&labelColor=grey)](https://garnix.io/repo/camdenboren/alc-calc)
![Static Badge](https://img.shields.io/badge/Powered_by_Nix-grey?logo=nixOS&logoColor=white&logoSize=auto&style=for-the-badge)

</div>

> [!NOTE]
> This project is under active development and has not yet been released, but it's usable in its current form. **Expect behavioral changes**

## Motivation

Weight-based measurement is growing in popularity for many in the kitchen, so why not apply this improved accuracy and consistency to making drinks?

The original project ([Alcohol_Weight_Calculator]) is a dead-simple Python CLI for quickly calculating the weight of ingredients in an alcoholic beverage. It has served me well, but it has notable limitations

- No input validation (shameful, I know)
- Inability to add nonalcoholic ingredients
- An emphasis on broadly categorizing ingredients handicaps precision when an ingredient's actual density is already known
- No GUI

The first three have relatively simple solutions that I could implement in a weekend, but the thought of retrofitting yet another Python GUI failed to excite me

Enter [GPUI], the UI Framework from the Zed team

- Tailwind's ease-of-use
- Rust's speed and safety
- First-class hardware acceleration

Rust rewrite with an undocumented framework it is :moyai:

## Install

### Nix

Add the following to your `flake.nix`

```nix
inputs = {
  nixpkgs = {
    url = "github:nixos/nixpkgs/nixos-unstable";
  };
  alc-calc = {
    url = "github:camdenboren/alc-calc";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  ...
}
```

Then, add alc-calc to your packages

> For system wide installation in `configuration.nix`

```nix
environment.systemPackages = with pkgs; [
  inputs.alc-calc.packages.${system}.default
];
```

> For user level installation in `home.nix`

```nix
home.packages = with pkgs; [
  inputs.alc-calc.packages.${system}.default
];
```

### Non-Nix

Once released, app bundles will be distributed in the [Releases] page. Download the correct bundle for your OS/distro and follow the standard installation procedures

Until then, the only way to install alc-calc is to first build it from source by following the manual instructions in [CONTRIBUTING]

> [!NOTE]
> Though I sign both `alc-calc.app` and `alc-calc.dmg` for macOS users, you'll still need to whitelist alc-calc before installing since I'm not paying $99 to notarize binaries for something no one else uses. This can be done by attempting to open `alc-calc.dmg` then navigating to: `System Settings -> Privacy and Security -> Security`, and clicking: `Open Anyway`. Repeat this step once you attempt to run alc-calc after installing, and then you should be able to run alc-calc like normal

> [!NOTE]
> I do not sign `alc-calc_*_x64-setup.exe` for Windows users for the same reason, so you'll need to click `More Info` before installing

## Contributing

[CONTRIBUTING]

## License

[GPLv3]

[Alcohol_Weight_Calculator]: https://github.com/camdenboren/Alcohol_Weight_Calculator
[GPUI]: https://www.gpui.rs/
[Releases]: https://github.com/camdenboren/alc-calc/releases
[CONTRIBUTING]: .github/CONTRIBUTING.md
[GPLv3]: COPYING
