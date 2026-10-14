# SPDX-FileCopyrightText: Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

{
  nixosModule,
  pkgs,
}:

let
  support = import ./support.nix { inherit pkgs; };
in
pkgs.testers.runNixOSTest {
  name = "alc-calc-web";

  nodes.machine = { ... }: {
    imports = [ nixosModule ];

    virtualisation = {
      cores = 2;
      diskSize = 4096;
      memorySize = 4096;
    };

    users.users.cypress = {
      isNormalUser = true;
      group = "users";
    };

    services.alc-calc = {
      enable = true;
      domain = "alc-calc.test";
      nginx.enable = true;
      nginx.virtualHost.listen = [
        {
          addr = "127.0.0.1";
          port = 8080;
        }
      ];
    };
  };

  testScript = ''
    machine.start()
    machine.wait_for_unit("nginx.service")
    machine.wait_for_open_port(8080)
    machine.wait_until_succeeds(
        "${pkgs.curl}/bin/curl --fail --silent "
        "http://127.0.0.1:8080/index.html"
    )
    machine.succeed(
        "runuser -u cypress -- ${support.cypress}/bin/alc-calc-cypress"
    )
  '';
}
