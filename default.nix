{ nixpkgs ? import <nixpkgs> { } }:

let
  rustOverlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pinnedPkgs = nixpkgs.fetchFromGitHub {
    owner  = "NixOS";
    repo   = "nixpkgs";
    rev    = "11cb3517b3af6af300dd6c055aeda73c9bf52c48";
    hash   = "sha256-rWtXrcIzU5wm/C8F9LWvUfBGu5U5E7cFzPYT1pHIJaQ=";
  };
  pkgs = import pinnedPkgs {
    overlays = [ (import rustOverlay) ];
  };
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      rust-bin.stable."1.89.0".default
      rust-analyzer
    ];

    RUST_BACKTRACE = 1;
  }
