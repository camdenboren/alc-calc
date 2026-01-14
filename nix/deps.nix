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
      bundle-mac
      cargo
      create-dmg
    ]
    ++ lib.optionals stdenv.hostPlatform.isLinux [
      bundle-linux
    ];

  dev =
    with pkgs;
    [
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
      build
      format
    ]
    ++ (with nodePackages; [
      prettier
    ]);

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
