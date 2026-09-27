# SPDX-FileCopyrightText: Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

{ pkgs }:

let
  rustNightly = pkgs.rust-bin.nightly.latest.default.override {
    extensions = [ "rust-src" ];
    targets = [ "wasm32-unknown-unknown" ];
  };
in
{
  build = {
    os =
      with pkgs;
      lib.optionals stdenv.hostPlatform.isLinux [
        fontconfig
        libxkbcommon
        libxcb
        libx11
        wayland
      ]
      ++ lib.optionals stdenv.hostPlatform.isDarwin [
        apple-sdk_15
        (darwinMinVersionHook "12.3")
      ];

    web = with pkgs; [ fontconfig ];
  };

  dev = {
    os = with pkgs; [
      boxes
      rustc
      cargo
      cargo-bundle
      cargo-edit
      rust-analyzer
      rustfmt
      taplo
      nixfmt
      clippy
      prettier
      build
      format
    ];

    web = with pkgs; [
      boxes
      rustNightly
      trunk
    ];

    bundle =
      with pkgs;
      [
        boxes
      ]
      ++ lib.optionals stdenv.hostPlatform.isDarwin [
        bundle-mac
        cargo
        create-dmg
      ]
      ++ lib.optionals stdenv.hostPlatform.isLinux [
        bundle-linux
      ];
  };

  run = {
    os =
      with pkgs;
      [
        pkg-config
      ]
      ++ lib.optionals stdenv.hostPlatform.isDarwin [
        cargo-bundle
        makeBinaryWrapper
      ];

    web = with pkgs; [
      binaryen
      pkg-config
      trunk
      wasm-bindgen-cli_0_2_127
    ];
  };

  # keep nightly in one place so the web devShell and package stay in sync
  inherit rustNightly;
}
