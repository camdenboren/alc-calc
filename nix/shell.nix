# SPDX-FileCopyrightText: Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

{ pkgs, deps }:

{
  default = pkgs.mkShell {
    packages = deps.dev.os;
    buildInputs = deps.build.os;
    nativeBuildInputs = deps.run.os;
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

  web = pkgs.mkShell {
    packages = deps.dev.web;
    buildInputs = deps.build.web;
    nativeBuildInputs = deps.run.web;
    env.RUST_SRC_PATH = "${deps.rustNightly}/lib/rustlib/src/rust/library";
    env.TRUNK_TOOLS_WASM_OPT = "version_${pkgs.binaryen.version}";

    shellHook = ''
      echo -e "\nalc-calc web DevShell via Nix Flake\n"

      echo -e "┌────────────────────────┐"
      echo -e "│    Useful Commands     │"
      echo -e "├────────┬───────────────┤"
      echo -e "│ Build  │ $ trunk build │"
      echo -e "│ Serve  │ $ trunk serve │"
      echo -e "│ Clean  │ $ trunk clean │"
      echo -e "└────────┴───────────────┘"
    '';
  };

  bundle = pkgs.mkShell {
    packages = deps.dev.bundle;
    env.CUR_OS = if pkgs.stdenv.hostPlatform.isDarwin then "mac" else "linux";

    shellHook = ''
      echo -e "\nalc-calc bundle DevShell via Nix Flake\n"

      if test -f .env; then
        set -a
        source .env
        set +a
      fi

      echo -e "┌─────────────────────────┐"
      echo -e "│     Useful Commands     │"
      echo -e "├────────┬────────────────┤"
      echo -e "│ Bundle │ $ bundle-$(printf %-5s $CUR_OS | tr ' ' " ") │"
      echo -e "└────────┴────────────────┘"
    '';
  };
}
