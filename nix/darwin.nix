# SPDX-FileCopyrightText: Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

# nix-darwin provides no nginx or caddy modules, so this serves the wasm
# build directly via launchd instead of the virtual hosts in nix/nixos.nix.
{ self }:
{
  config,
  lib,
  pkgs,
  ...
}:

let
  logFile = "/var/log/alc-calc.log";
  cfg = config.services.alc-calc;
in
{
  options.services.alc-calc = {
    enable = lib.mkEnableOption "alc-calc";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.web;
      defaultText = lib.literalExpression "inputs.alc-calc.packages.\${pkgs.stdenv.hostPlatform.system}.web";
      description = "The alc-calc wasm build to serve.";
    };

    host = lib.mkOption {
      type = lib.types.str;
      default = "127.0.0.1";
      description = "Listening host for alc-calc.";
    };

    port = lib.mkOption {
      type = lib.types.port;
      default = 8080;
      description = "Listening port for alc-calc.";
    };
  };

  config = lib.mkIf cfg.enable {
    launchd.daemons.alc-calc = {
      command = "${pkgs.static-web-server}/bin/static-web-server --host ${cfg.host} --port ${toString cfg.port} --root ${cfg.package} --page-fallback ${cfg.package}/index.html";

      serviceConfig = {
        KeepAlive = true;
        ProcessType = "Background";
        RunAtLoad = true;
        StandardErrorPath = logFile;
        StandardOutPath = logFile;
        WorkingDirectory = "/tmp";
      };
    };
  };
}
