{
  description = "Rust development template using fenix";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, fenix, ... }:
    utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ fenix.overlays.default ];
          };
          toolchain = pkgs.fenix.complete;
          buildInputs = with pkgs; [
            webkitgtk
          ];
        in
        rec
        {
          devShells.default =
            let
              target = "wasm32-unknown-unknown";
            in
            pkgs.mkShell {

              # Use nightly cargo & rustc provided by fenix. Add for packages for the dev shell here
              buildInputs = with pkgs;
                [
                  (with pkgs.fenix; combine [
                    complete.cargo
                    complete.rustc
                    complete.rust-src
                    complete.clippy
                    complete.rustfmt
                    targets.${target}.latest.rust-std
                  ])
                  (with nodePackages; [
                    tailwindcss
                    web-ext
                  ])
                  pkg-config
                  openssl
                  wasm-pack
                  just
                ] ++ buildInputs;

              # Specify the rust-src path (many editors rely on this)
              RUST_SRC_PATH = "${toolchain.rust-src}/lib/rustlib/src/rust/library";
            };
        }
      );
}
