{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, fenix, advisory-db }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          system = "x86_64-linux";
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "x86_64-unknown-linux-gnu" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        commonArgs = {
          src = craneLib.cleanCargoSource ./.;

          CARGO_BUILD_PACKAGE = "pfp";
          CARGO_BUILD_TARGET = "x86_64-unknown-linux-gnu";
          # CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";

          buildInputs = with pkgs; [
            # Add extra build inputs here, etc.
            # (TLDR: Run time)
          ];

          nativeBuildInputs = with pkgs; [
            # Add extra native build inputs here, etc.
            # (TLDR: Build time)
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          # Additional arguments specific to this derivation can be added here.
          # Be warned that using `//` will not do a deep copy of nested
          # structures
          pname = "pfp-deps";
        });

        pfp = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });

        pfpDoc = craneLib.cargoDoc (commonArgs // {
          inherit cargoArtifacts;
        });

        checkClippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        });

        checkTests = craneLib.cargoNextest (commonArgs // {
          inherit cargoArtifacts;
        });

        checkFmt = craneLib.cargoFmt (commonArgs // {
          inherit cargoArtifacts;
        });

        checkAudit = craneLib.cargoAudit (commonArgs // {
          inherit cargoArtifacts advisory-db;
        });
      in { 
        packages.default = pfp;

        checks = {
          inherit
            # Build the crate as part of `nix flake check` for convenience
            pfp
            checkFmt
            checkClippy
            checkTests;
        };
      });
}

