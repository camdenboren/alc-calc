# SPDX-FileCopyrightText: Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

{ pkgs }:

{
  build =
    with pkgs;
    lib.optionals stdenv.hostPlatform.isLinux [
      libxkbcommon
      xorg.libxcb
      xorg.libX11
      wayland
    ]
    ++ lib.optionals stdenv.hostPlatform.isDarwin [
      apple-sdk_15
      (darwinMinVersionHook "12.3")
    ];

  bundle =
    with pkgs;
    [
      boxes
    ]
    ++ lib.optionals stdenv.hostPlatform.isDarwin [
      cargo
      create-dmg
    ];

  dev = with pkgs; [
    rustc
    cargo
    cargo-bundle
    rust-analyzer
    rustfmt
    taplo
    clippy
    build
    format
  ];

  run =
    with pkgs;
    [
      pkg-config
    ]
    ++ lib.optionals stdenv.hostPlatform.isDarwin [
      cargo-bundle
      makeBinaryWrapper
    ];
}
