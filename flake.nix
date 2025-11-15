{
  description = "WWL Rust Bindings";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rustfmt"
            "clippy"
          ];
        };

        wwl = pkgs.python3Packages.buildPythonPackage rec {
          pname = "wwl";
          version = "0.1.2";

          src = pkgs.fetchPypi {
            inherit pname version;
            sha256 = "sha256-3yzh+NV97ohwbENQq5/yLUP7n+dZb2+vtAimWEWlw+Y=";
          };

          format = "setuptools";

          doCheck = false;

          propagatedBuildInputs = with pkgs.python3Packages; [
            cython
            numpy
            pot
            igraph
            scikit-learn
            scipy
          ];

          meta = {
            description = "Wasserstein Weisfeiler-Lehman Graph Kernels";
            license = pkgs.lib.licenses.mit;
          };
        };

        env = pkgs.python3.withPackages (
          ps: with ps; [
            cython
            numpy
            pot
            igraph
            scikit-learn
            scipy
            wwl
          ]
        );

      in
      {
        packages.python.wwl = wwl;
        packages.python.env = env;

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            env
            libffi
            maturin
            openssl
            pkg-config
            stdenv.cc.cc.lib
            toolchain
            zlib
          ];

          shellHook = ''
            export PYTHON_SYS_EXECUTABLE=${env}/bin/python
            export PYO3_PYTHON=${env}/bin/python
            export LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib:${pkgs.zlib}/lib:${pkgs.libffi}/lib:$LD_LIBRARY_PATH"

            if ![ ${env}/bin/python -c "import wwl" 2>/dev/null ]; then
              echo "wwl not found"
            fi
          '';
        };

        packages.wwl = pkgs.rustPlatform.buildRustPackage {
          pname = "wwl";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            env
            maturin
            pkg-config
            toolchain
          ];

          buildInputs = with pkgs; [
            openssl
          ];

          preBuild = ''
            export PYTHON_SYS_EXECUTABLE=${env}/bin/python
            export PYO3_PYTHON=${env}/bin/python
          '';
        };
      }
    );
}
