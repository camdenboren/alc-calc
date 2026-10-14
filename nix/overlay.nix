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
in
{
  build = writePatchedScript "build";
  format = writePatchedScript "format";
  "bundle-${curOS}" = writePatchedScript "bundle-${curOS}";

  cypress = prev.cypress.overrideAttrs (o: {
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
