# SPDX-FileCopyrightText: Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

# Building `.#checks.aarch64-darwin.web` evaluates and builds this
# configuration. CI additionally activates it on an ephemeral macOS runner and
# exercises the wasm build and Cypress via `testScript`.
{
  nix-darwin,
  module,
  system,
  activate ? false,
}:

nix-darwin.lib.darwinSystem {
  inherit system;
  modules = [
    module
    (
      { pkgs, ... }:
      let
        support = import ./support.nix { inherit pkgs; };
        webLog = "/var/log/alc-calc.log";
      in
      {
        system.stateVersion = 5;
        system.primaryUser = "runner";
        nix.enable = !activate;
        nixpkgs.overlays = [ (import ../overlay.nix) ];

        # Cypress's bundled Electron is flagged insecure at this nixpkgs
        # revision; the wasm smoke test needs the Cypress binary to run.
        nixpkgs.config.permittedInsecurePackages = [ "cypress-15.19.0" ];

        services.alc-calc = {
          enable = true;
          port = 8080;
        };

        environment.systemPackages = [
          (pkgs.writeShellScriptBin "testScript" ''
            set -euo pipefail

            trap 'sudo launchctl print system/org.nixos.alc-calc || true; sudo tail -n 100 ${webLog} || true' ERR

            sudo launchctl print system/org.nixos.alc-calc

            for _ in {1..60}; do
              if /usr/bin/curl --fail --silent http://127.0.0.1:8080/index.html >/dev/null; then
                break
              fi
              sleep 1
            done

            ${support.cypress}/bin/alc-calc-cypress
          '')
        ];
      }
    )
  ];
}
