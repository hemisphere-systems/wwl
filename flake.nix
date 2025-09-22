{
  description = "WWL Rust bindings with PyO3";

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
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rustfmt"
            "clippy"
          ];
        };

        # Build WWL package from PyPI
        wwlPackage = pkgs.python3Packages.buildPythonPackage rec {
          pname = "wwl";
          version = "0.1.2";

          src = pkgs.fetchPypi {
            inherit pname version;
            sha256 = "sha256-3yzh+NV97ohwbENQq5/yLUP7n+dZb2+vtAimWEWlw+Y=";
          };

          format = "setuptools";

          doCheck = false; # Skip tests for faster builds

          propagatedBuildInputs = with pkgs.python3Packages; [
            cython
            numpy
            scipy
            scikit-learn
            python-igraph
            pot
          ];

          meta = {
            description = "Wasserstein Weisfeiler-Lehman Graph Kernels";
            license = pkgs.lib.licenses.mit;
          };
        };

        # Python environment with WWL and all dependencies
        pythonEnv = pkgs.python3.withPackages (
          ps: with ps; [
            wwlPackage
            numpy
            scipy
            scikit-learn
            python-igraph
            pot
            cython
          ]
        );

      in
      {
        # Export the WWL package for use in other flakes
        packages.wwl = wwlPackage;
        packages.python-env = pythonEnv;

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pythonEnv
            pkg-config
            openssl
            maturin
            # Add additional libraries that might be needed
            stdenv.cc.cc.lib
            zlib
            libffi
          ];

          shellHook = ''
            # Set up Python environment for PyO3
            export PYTHON_SYS_EXECUTABLE=${pythonEnv}/bin/python
            export PYO3_PYTHON=${pythonEnv}/bin/python

            # Ensure library paths are set
            export LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib:${pkgs.zlib}/lib:${pkgs.libffi}/lib:$LD_LIBRARY_PATH"

            # Verify WWL is available
            if ${pythonEnv}/bin/python -c "import wwl" 2>/dev/null; then
              echo "✅ WWL package is available"
            else
              echo "⚠️  WWL package not found"
            fi

            echo ""
            echo "🚀 WWL Rust development environment ready!"
            echo "Python: ${pythonEnv}/bin/python"
            echo "Rust: $(which rustc)"
            echo "Tools: maturin, cargo"
          '';
        };

        packages.rust-wwl = pkgs.rustPlatform.buildRustPackage {
          pname = "wwl-rust";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            rustToolchain
            pythonEnv
            maturin
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
          ];

          preBuild = ''
            export PYTHON_SYS_EXECUTABLE=${pythonEnv}/bin/python
            export PYO3_PYTHON=${pythonEnv}/bin/python
          '';
        };
      }
    );
}

