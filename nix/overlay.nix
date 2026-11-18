# SPDX-FileCopyrightText: Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

final: prev:
let
  # read script by name and patch bash shebang for nix users
  # see https://ertt.ca/nix/shell-scripts/
  writePatchedScript =
    name:
    (prev.writeScriptBin name (builtins.readFile ../script/${name})).overrideAttrs (old: {
      buildCommand = "${old.buildCommand}\n patchShebangs $out";
    });
  curOS = if prev.stdenv.hostPlatform.isDarwin then "mac" else "linux";
  availableBinaries = {
    x86_64-linux = {
      platform = "linux-x64";
      hash = "sha256-zELSx/th2R4Zs8FYndGWk2a+gDxiP/yeDF1B4AbhdKI=";
    };
    aarch64-linux = {
      platform = "linux-arm64";
      hash = "sha256-ssm4WVzDMTo5cWDJaPoEddUmpHK1oh1453El5xexzrQ=";
    };
    aarch64-darwin = {
      platform = "darwin-arm64";
      hash = "sha256-p0hK/kqxcgQlGz8MLByOxrqnkJudHPjO3sR8h9Wmn+c=";
    };
  };
  inherit (prev.stdenv.hostPlatform) system;
  binary =
    availableBinaries.${system} or (throw "cypress: No binaries available for system ${system}");
  inherit (binary) platform hash;
in
{
  build = writePatchedScript "build";
  format = writePatchedScript "format";
  "bundle-${curOS}" = writePatchedScript "bundle-${curOS}";

  cypress = prev.cypress.overrideAttrs (o: rec {
    version = "16.3.0";
    meta = o.meta // {
      # The inherited warning concerns Electron 37.6.0;
      # Cypress now bundles an unaffected version
      knownVulnerabilities = [ ];
    };
    src = prev.fetchzip {
      url = "https://cdn.cypress.io/desktop/${version}/${platform}/cypress.zip";
      inherit hash;
      stripRoot = !prev.stdenv.hostPlatform.isDarwin;
    };
    installPhase =
      o.installPhase or ""
      +
        # cli usage via npx works on macOS once binary_state.json is writable
        # CYPRESS_SKIP_VERIFY=true didnt' work for me
        prev.lib.optionalString prev.stdenv.hostPlatform.isDarwin ''
          cp $out/binary_state.json $out/opt/cypress/binary_state.json
          chmod +w $out/opt/cypress/binary_state.json
        '';
  });
}
