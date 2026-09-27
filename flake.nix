# SPDX-FileCopyrightText: Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

{
  description = "alc-calc DevShell and Package via Nix Flake";

  nixConfig.bash-prompt = ''\n\[\033[1;31m\][devShell:\w]\$\[\033[0m\] '';

  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, rust-overlay, ... }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        function:
        nixpkgs.lib.genAttrs supportedSystems (
          system:
          function rec {
            pkgs = import nixpkgs {
              inherit system;
              overlays = [
                (import ./nix/overlay.nix)
                (import rust-overlay)
              ];
            };
            deps = (import ./nix/deps.nix { inherit pkgs; });
          }
        );
    in
    {
      devShells = forEachSupportedSystem (import ./nix/shell.nix);
      packages = forEachSupportedSystem (import ./nix/package.nix);
    };
}
