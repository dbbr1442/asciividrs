{
    description = "A rust program that can theoretically play any video similar to bad apple but is preconfigured to play such";

    inputs = {
        nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
        rust-overlay.url = "github:oxalica/rust-overlay";
        flake-utils.url  = "github:numtide/flake-utils";
    };

    outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
    let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
            inherit system overlays;
        };

        

        asciividrs = (with pkgs; rustPlatform.buildRustPackage rec {
            pname = "asciividrs";
            version = "0.1";
            nativeBuildInputs = with pkgs; [ pkg-config alsa-lib.dev ];
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.lib.cleanSource ./.;
            PKG_CONFIG_PATH = "${pkgs.alsa-lib.dev}/lib/pkgconfig";
        }); 
    in
    {
        devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
                cmake
                alsa-lib
                pkg-config
                rust-bin.beta.latest.default
            ];
        };

        defaultPackage = asciividrs; 
      }
    );
}
