# SPDX-FileCopyrightText: Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

{ pkgs }:

pkgs.writeShellScriptBin "build" ''
  set -o pipefail
  box() { ${pkgs.boxes}/bin/boxes -d ansi -s $(tput cols); }
  failed() {
    echo -e "\n\033[1;31mBuild failed.\033[0m"
    exit 1
  }
  trap 'failed' ERR

  echo -e "\033[1;33mcargo...\033[0m"
  cargo check 2> /dev/null | box

  echo -e "\n\033[1;33mbuild...\033[0m"
  nix build 2> /dev/null | box

  echo -e "\n\033[1;32mBuild succeeded.\033[0m"
''
