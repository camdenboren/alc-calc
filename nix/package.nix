# SPDX-FileCopyrightText: Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

{ pkgs, deps }:

{
  default = pkgs.rustPlatform.buildRustPackage {
    pname = "alc-calc";
    version = "0.1.0";
    src = ../.;

    cargoHash = "sha256-fzfe2ig/fWepd0XdvzQBONMFFTVcongSIdJ201DY7nA=";
    useFetchCargoVendor = true;
    buildInputs = deps.build;
    nativeBuildInputs = deps.run;
    buildFeatures = with pkgs; lib.optionals stdenv.hostPlatform.isDarwin [ "gpui/runtime_shaders" ];

    env.LIBCLANG_PATH =
      with pkgs;
      lib.optionalString stdenv.hostPlatform.isDarwin "${lib.getLib llvmPackages.libclang}/lib";

    # darwin ci checks are flaky due to missing ScreenCaptureKit
    doCheck = (!pkgs.stdenv.hostPlatform.isDarwin);

    # simplified adaptation of zed's installPhase
    # https://github.com/NixOS/nixpkgs/blob/50b354db88ed70cf031b6986a516fd5564559ea1/pkgs/by-name/ze/zed-editor/package.nix
    installPhase =
      ''
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
        mv target/release/alc-calc $out/bin/
        mv $app_path $out/Applications/
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
