let
  # Update as necessary from https://status.nixos.org/...
  # The URL takes the form https://github.com/NixOS/nixpkgs/archive/COMMIT_HASH.tar.gz
  pkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/617579a787259b9a6419492eaac670a5f7663917.tar.gz") {};
in
  pkgs.mkShell {
    buildInputs = [
      pkgs.cargo
      pkgs.rustc
      pkgs.rustfmt
      pkgs.clippy
      pkgs.rust-analyzer
    ];

    # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela.
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  }
