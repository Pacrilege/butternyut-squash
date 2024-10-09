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

	    (vscode-with-extensions.override {
	      vscode = vscodium;
	      vscodeExtensions = with vscode-extensions; [
		# Nix
		bbenoist.nix
		kamadorueda.alejandra

		# Rust
		rust-lang.rust-analyzer
		tamasfe.even-better-toml
		serayuzgur.crates

		# Writing
		yzhang.markdown-all-in-one
		nvarner.typst-lsp
		stkb.rewrap

		# General
		eamodio.gitlens
		asvetliakov.vscode-neovim
	      ];
	    })
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
