# SPDX-FileCopyrightText: Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

{ pkgs }:

let
  cypressCli = pkgs.buildNpmPackage {
    pname = "alc-calc-web";
    version = "0.1.0";
    src = ../../web;

    env.CYPRESS_INSTALL_BINARY = 0;
    npmDepsHash = "sha256-BE1ehuwS74Z1Fe05GvmsAonrJf1JoF9+Z+NyGAhvUas=";
    dontNpmBuild = true;

    installPhase = ''
      runHook preInstall

      runner="$out/lib/node_modules/alc-calc-web"
      mkdir -p "$runner/cypress"
      cp -r node_modules "$runner/"
      cp ${../../web/package.json} "$runner/package.json"
      cp ${../../web/package-lock.json} "$runner/package-lock.json"
      cp ${../../web/tsconfig.json} "$runner/tsconfig.json"
      cp -r ${../../web/cypress/e2e} "$runner/cypress/e2e"
      cp -r ${../../web/cypress/support} "$runner/cypress/support"
      cp ${../../web/cypress.config.ts} "$runner/cypress.config.ts"

      runHook postInstall
    '';
  };

  cypressBinary =
    if pkgs.stdenv.hostPlatform.isDarwin then
      "${pkgs.cypress}/opt/cypress/Cypress.app/Contents/MacOS/Cypress"
    else
      "${pkgs.cypress}/bin/Cypress";

  cypress = pkgs.writeShellApplication {
    name = "alc-calc-cypress";
    runtimeInputs = [
      pkgs.coreutils
      pkgs.nodejs
    ]
    ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isLinux [ pkgs.xvfb-run ];
    text = ''
      test_home=$(mktemp -d)
      trap 'rm -rf "$test_home"' EXIT

      export CI=1
      export CYPRESS_INSTALL_BINARY=0
      export CYPRESS_RUN_BINARY=${cypressBinary}
      export HOME="$test_home"
      mkdir -p /tmp/alc-calc-cypress

      runner=${cypressCli}/lib/node_modules/alc-calc-web
      test_project="$test_home/project"
      mkdir -p "$test_project"
      cp -r "$runner/cypress" "$test_project/cypress"
      cp "$runner/cypress.config.ts" "$test_project/cypress.config.ts"
      cp "$runner/tsconfig.json" "$test_project/tsconfig.json"
      chmod -R u+w "$test_project"
      ln -s "$runner/node_modules" "$test_project/node_modules"

      ${pkgs.lib.optionalString pkgs.stdenv.hostPlatform.isLinux "xvfb-run --auto-servernum "}node \
        "$runner/node_modules/cypress/bin/cypress" \
        run \
        --browser electron \
        --config baseUrl=http://127.0.0.1:8080 \
        --project "$test_project"
    '';
  };
in
{
  inherit cypress;
}
