# SPDX-FileCopyrightText: 2025 Camden Boren
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

    # darwin ci checks are flaky due to missing ScreenCaptureKit
    doCheck = (!pkgs.stdenv.hostPlatform.isDarwin);
    postFixup =
      with pkgs;
      lib.optionalString stdenv.hostPlatform.isLinux ''
        patchelf --add-rpath ${wayland}/lib $out/bin/*
        patchelf --add-rpath ${vulkan-loader}/lib $out/bin/*
      '';
    env.LIBCLANG_PATH =
      with pkgs;
      lib.optionalString stdenv.hostPlatform.isDarwin "${lib.getLib llvmPackages.libclang}/lib";
  };
}
