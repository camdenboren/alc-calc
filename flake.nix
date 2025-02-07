# SPDX-FileCopyrightText: 2025 Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

{
  description = "alc-calc Development Environment via Nix Flake";

  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
  };

  outputs =
    { nixpkgs, ... }:
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
            pkgs = nixpkgs.legacyPackages.${system}.extend (import ./nix/overlay.nix { inherit pkgs; });
            deps =
              with pkgs;
              lib.optionals stdenv.hostPlatform.isLinux [
                libxkbcommon
                xorg.libxcb
                wayland
              ]
              ++ lib.optionals stdenv.hostPlatform.isDarwin [
                apple-sdk_15
                (darwinMinVersionHook "12.3")
              ];
          }
        );
    in
    {
      devShells = forEachSupportedSystem (
        { pkgs, deps }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              bashInteractive
              rustc
              cargo
              rust-analyzer
              rustfmt
              build
              format
            ];

            buildInputs = deps;
            LD_LIBRARY_PATH =
              with pkgs;
              lib.optionals stdenv.hostPlatform.isLinux (
                lib.makeLibraryPath [
                  wayland
                  vulkan-loader
                ]
              );

            shellHook = import ./nix/shellHook.nix;
          };
        }
      );

      packages = forEachSupportedSystem (
        { pkgs, deps }:
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "alc-calc";
            version = "0.1.0";
            src = ./.;

            cargoHash = "sha256-0mCN6uO10NJi/2+cWxkl2ZDa8KwQlK8Bn1fQXfY9ReY=";
            useFetchCargoVendor = true;
            buildInputs = deps;
            buildFeatures = with pkgs; lib.optionals stdenv.hostPlatform.isDarwin [ "gpui/runtime_shaders" ];
            postFixup =
              with pkgs;
              lib.optionalString stdenv.hostPlatform.isLinux ''
                patchelf --add-rpath ${wayland}/lib $out/bin/*
                patchelf --add-rpath ${vulkan-loader}/lib $out/bin/*
              '';

            meta = {
              description = "";
              maintainers = [ "camdenboren" ];
            };
          };
        }
      );
    };
}
