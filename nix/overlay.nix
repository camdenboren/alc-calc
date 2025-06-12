# SPDX-FileCopyrightText: Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

{ pkgs, ... }:

(final: prev: {
  build = pkgs.callPackage ./build.nix { };
  format = pkgs.callPackage ./format.nix { };
})
