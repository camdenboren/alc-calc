# SPDX-FileCopyrightText: 2025 Camden Boren
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
  dev = with pkgs; [
    bashInteractive
    rustc
    cargo
    rust-analyzer
    rustfmt
    taplo
    clippy
    build
    format
  ];
  run = with pkgs; [ pkg-config ];
}
