# SPDX-FileCopyrightText: Camden Boren
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

      echo -e "┌───────────────────────┐"
      echo -e "│    Useful Commands    │"
      echo -e "├────────┬──────────────┤"
      echo -e "│ Build  │ $ build      │"
      echo -e "│ Format │ $ format     │"
      echo -e "│ Run    │ $ cargo run  │"
      echo -e "│ Test   │ $ cargo test │"
      echo -e "└────────┴──────────────┘"
    '';
  };

  bundle = pkgs.mkShell {
    packages = deps.bundle;
    env.CUR_OS = if pkgs.stdenv.hostPlatform.isDarwin then "mac" else "linux";

    shellHook = ''
      echo -e "\nalc-calc bundle DevShell via Nix Flake\n"

      if test -f .env; then
        set -a
        source .env
        set +a
      fi

      echo -e "┌───────────────────────────────────────┐"
      echo -e "│            Useful Commands            │"
      echo -e "├────────┬──────────────────────────────┤"
      echo -e "│ Chmod  │ $ chmod +x ./os/bundle-$(printf %-5s $CUR_OS | tr ' ' " ") │"
      echo -e "│ Bundle │ $ ./os/bundle-$(printf %-5s $CUR_OS | tr ' ' " ")          │"
      echo -e "└────────┴──────────────────────────────┘"
    '';
  };
}
