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
    nix-darwin = {
      url = "github:nix-darwin/nix-darwin";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      nix-darwin,
      ...
    }:
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
              # Cypress's bundled Electron is flagged insecure at this nixpkgs
              # revision; the wasm smoke test needs the Cypress binary to run.
              config.permittedInsecurePackages = [ "cypress-15.19.0" ];
            };
            deps = (import ./nix/deps.nix { inherit pkgs; });
          }
        );
    in
    {
      devShells = forEachSupportedSystem (import ./nix/shell.nix);
      packages = forEachSupportedSystem (import ./nix/package.nix);
      nixosModules = rec {
        default = alc-calc;
        alc-calc = import ./nix/nixos.nix { inherit self; };
      };
      darwinModules = rec {
        default = alc-calc;
        alc-calc = import ./nix/darwin.nix { inherit self; };
      };
      checks = forEachSupportedSystem (
        { pkgs, ... }:
        nixpkgs.lib.optionalAttrs pkgs.stdenv.hostPlatform.isLinux {
          web = import ./nix/tests/nixos.nix {
            inherit pkgs;
            nixosModule = self.nixosModules.default;
          };
        }
        # Darwin testing approach from https://github.com/Mic92/fast-nix-gc
        // nixpkgs.lib.optionalAttrs pkgs.stdenv.hostPlatform.isDarwin {
          web =
            (import ./nix/tests/darwin.nix {
              inherit nix-darwin;
              module = self.darwinModules.default;
              system = pkgs.stdenv.hostPlatform.system;
            }).system;
        }
      );
      darwinConfigurations.ci = import ./nix/tests/darwin.nix {
        inherit nix-darwin;
        module = self.darwinModules.default;
        system = "aarch64-darwin";
        activate = true;
      };
    };
}
