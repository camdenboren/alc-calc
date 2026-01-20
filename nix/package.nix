# SPDX-FileCopyrightText: Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

{ pkgs, deps }:

{
  default = pkgs.rustPlatform.buildRustPackage {
    pname = "alc-calc";
    version = "0.1.0";
    src = ../.;

    cargoHash = "sha256-jSQe/ZWsYWNxM8IIQMRT5DhcKpd4+q4uJkYsoEDCPEY=";
    buildInputs = deps.build;
    nativeBuildInputs = deps.run;
    buildFeatures = with pkgs; lib.optionals stdenv.hostPlatform.isDarwin [ "runtime_shaders" ];

    env.LIBCLANG_PATH =
      with pkgs;
      lib.optionalString stdenv.hostPlatform.isDarwin "${lib.getLib llvmPackages.libclang}/lib";

    # darwin ci checks are flaky due to missing ScreenCaptureKit
    doCheck = (!pkgs.stdenv.hostPlatform.isDarwin);

    # simplified adaptation of zed's installPhase
    # https://github.com/NixOS/nixpkgs/blob/50b354db88ed70cf031b6986a516fd5564559ea1/pkgs/by-name/ze/zed-editor/package.nix
    installPhase = ''
      runHook preInstall
      release_target="target/${pkgs.stdenv.hostPlatform.rust.cargoShortTarget}/release"
    ''
    + pkgs.lib.optionalString pkgs.stdenv.hostPlatform.isDarwin ''
      # cargo-bundle expects the binary in target/release
      mv $release_target/alc-calc target/release/alc-calc

      # skip build to prevent supposed missing metal shaders error
      export CARGO_BUNDLE_SKIP_BUILD=true
      app_path=$(cargo bundle --release | xargs)

      # moving the binary to $out/bin works, but a wrapper might allow the icon to show
      mkdir -p $out/Applications $out/bin
      mv $app_path $out/Applications/
      makeWrapper $out/Applications/alc-calc.app/Contents/MacOS/alc-calc $out/bin/alc-calc
    ''
    + pkgs.lib.optionalString pkgs.stdenv.hostPlatform.isLinux ''
      install -Dm755 $release_target/alc-calc $out/bin/alc-calc

      install -Dm644 $src/img/brand/app-icon@2x.png $out/share/icons/hicolor/1024x1024@2x/apps/alc-calc.png
      install -Dm644 $src/img/brand/app-icon.png $out/share/icons/hicolor/512x512/apps/alc-calc.png
      install -Dm644 $src/os/alc-calc.desktop $out/share/applications/alc-calc.desktop
    ''
    + ''
      runHook postInstall
    '';

    postFixup =
      with pkgs;
      lib.optionalString stdenv.hostPlatform.isLinux ''
        patchelf --add-rpath ${wayland}/lib $out/bin/*
        patchelf --add-rpath ${vulkan-loader}/lib $out/bin/*
      '';
  };
}
