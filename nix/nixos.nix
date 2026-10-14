# SPDX-FileCopyrightText: Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

{ self }:
{
  config,
  lib,
  pkgs,
  ...
}:

let
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

    domain = lib.mkOption {
      description = "Domain to use for the virtual host.";
      type = lib.types.str;
    };

    nginx = {
      enable = lib.mkEnableOption "a virtualhost to serve alc-calc through nginx";

      virtualHost = lib.mkOption {
        type = lib.types.attrs;
        default = { };
        example = lib.literalExpression ''
          {
            serverAliases = [ "alc-calc.''${config.networking.domain}" ];
          }
        '';
        description = "Extra configuration for the nginx virtual host of alc-calc.";
      };
    };

    caddy = {
      enable = lib.mkEnableOption "a virtualhost to serve alc-calc through caddy";

      virtualHost = lib.mkOption {
        type = lib.types.attrs;
        default = { };
        example = lib.literalExpression ''
          {
            serverAliases = [ "alc-calc.''${config.networking.domain}" ];
          }
        '';
        description = "Extra configuration for the caddy virtual host of alc-calc.";
      };
    };
  };

  config = lib.mkIf cfg.enable {
    services.nginx = lib.mkIf cfg.nginx.enable {
      enable = lib.mkDefault true;
      virtualHosts."${cfg.domain}" = lib.mkMerge [
        (lib.mapAttrsRecursive (_: lib.mkDefault) cfg.nginx.virtualHost)
        {
          root = lib.mkForce "${cfg.package}";

          locations."/" = {
            index = "index.html";
            tryFiles = "$uri $uri/ /index.html";
            extraConfig = ''
              add_header X-Frame-Options "SAMEORIGIN" always;
              add_header X-Content-Type-Options "nosniff" always;
              add_header X-XSS-Protection "1; mode=block" always;
            '';
          };
        }
      ];
    };

    services.caddy = lib.mkIf cfg.caddy.enable {
      enable = lib.mkDefault true;
      virtualHosts."${cfg.domain}" = lib.mkMerge [
        (lib.mapAttrsRecursive (_: lib.mkDefault) cfg.caddy.virtualHost)
        {
          hostName = lib.mkForce cfg.domain;
          extraConfig = ''
            root * ${cfg.package}
            try_files {path} /index.html
            file_server
          '';
        }
      ];
    };
  };
}
