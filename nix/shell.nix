# SPDX-FileCopyrightText: 2025 Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

{ pkgs, deps }:

{
  default = pkgs.mkShell {
    packages = deps.dev;
    buildInputs = deps.build;
    nativeBuildInputs = deps.run;
    LD_LIBRARY_PATH =
      with pkgs;
      lib.optionals stdenv.hostPlatform.isLinux (
        lib.makeLibraryPath [
          wayland
          vulkan-loader
        ]
      );

    shellHook = ''
      echo -e "\nalc-calc DevShell via Nix Flake\n"

      echo -e "┌─────────────────────────┐"
      echo -e "│     Useful Commands     │"
      echo -e "├─────────────────────────┤"
      echo -e "│ Build    │ $ build      │"
      echo -e "│ Format   │ $ format     │"
      echo -e "│ Run      │ $ cargo run  │"
      echo -e "│ Test     │ $ cargo test │"
      echo -e "└──────────┴──────────────┘"
    '';
  };
}
