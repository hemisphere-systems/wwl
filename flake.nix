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

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };

        pythonEnv = pkgs.python3.withPackages (ps: with ps; [
          numpy
          scipy
          scikit-learn
          pip
          setuptools
          wheel
          maturin
          pyo3
        ]);

        pythonEnvWithWWL = pkgs.python3.withPackages (ps: with ps; [
          numpy
          scipy
          scikit-learn
          pip
          setuptools
          wheel
          maturin
          pyo3
          cython
          python-igraph
          pot
        ]);

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pythonEnvWithWWL
            uv
            pkg-config
            openssl
            maturin
          ];

          shellHook = ''
            export PYTHON_SYS_EXECUTABLE=${pythonEnvWithWWL}/bin/python
            export PYO3_PYTHON=${pythonEnvWithWWL}/bin/python
            
            # Install wwl package using uv/pip
            echo "Installing wwl package..."
            ${pythonEnvWithWWL}/bin/pip install wwl
            
            echo "WWL Rust development environment ready!"
            echo "Python path: $(which python)"
            echo "Rust path: $(which rustc)"
            echo "Available tools: uv, maturin, cargo"
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "wwl-rust";
          version = "0.1.0";
          
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            rustToolchain
            pythonEnvWithWWL
            maturin
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
          ];

          preBuild = ''
            export PYTHON_SYS_EXECUTABLE=${pythonEnvWithWWL}/bin/python
            export PYO3_PYTHON=${pythonEnvWithWWL}/bin/python
          '';
        };
      });
}