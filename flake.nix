{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, systems, treefmt-nix, ... }: flake-parts.lib.mkFlake { inherit inputs; } {
    systems = import systems;
    imports = [ treefmt-nix.flakeModule ];
    perSystem = { self, config, pkgs, lib, system, ... }:
      let
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        nonRustDeps = with pkgs; [
          libiconv
          openssl
        ];
        rust-toolchain = pkgs.symlinkJoin {
          name = "rust-toolchain";
          paths = with pkgs; [
            rustc
            cargo
            rust-analyzer
            rustPlatform.rustcSrc
            pkg-config
          ];
        };
      in
      {
        # Rust package
        packages.default = pkgs.rustPlatform.buildRustPackage {
          inherit (cargoToml.package) name version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          buildInputs = nonRustDeps;
          nativeBuildInputs = [ rust-toolchain ];
        };

        # Rust dev environment
        devShells.default = pkgs.mkShell {
          inputsFrom = [ config.treefmt.build.devShell ];
          shellHook = ''
            # For rust-analyzer 'hover' tooltips to work.
            export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
          '';
          buildInputs = nonRustDeps;
          nativeBuildInputs = [ rust-toolchain ];
          RUST_BACKTRACE = 1;
        };

        # Add your auto-formatters here.
        # cf. https://numtide.github.io/treefmt/
        treefmt.config = {
          projectRootFile = "flake.nix";
          programs = {
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
          };
        };
      };
  };
}
