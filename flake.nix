{
  description = "Rust dev shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }: 
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;
      in {
        devShell = with pkgs; mkShell rec {
          nativeBuildInputs = [
            pkg-config
            (rustVersion.override { extensions = [ "rust-src" ]; })
            rust-analyzer
            clang
            mold
          ];
          buildInputs = [
            udev alsa-lib vulkan-loader
            xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
            libxkbcommon wayland # To use the wayland feature
	    gtk3 libGL
          ];
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      } 
    );
}
