{
  description = "A simple, dynamically typed, interpreted language. It is
  designed to be easy to learn and use. The Nana interpreter runs as a
  WebAssembly component.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, naersk }:
  let
    pkgs = nixpkgs.legacyPackages.aarch64-darwin;
    naersk' = pkgs.callPackage naersk {};
    deps = [
      # For build
      pkgs.rustup
      pkgs.cargo
      pkgs.wasmtime
      pkgs.wasm-tools

      pkgs.gnutar
      pkgs.gnugrep
      pkgs.cargo-component

      # For tests
      pkgs.fswatch

      # For project metrics
      pkgs.cloc
      pkgs.toybox
    ];
  in
  {
    packages.aarch64-darwin.default = naersk'.buildPackage {
      src = ./.;
      buildInputs = deps;
      cargoBuild = command: 
        nixpkgs.lib.strings.concatStrings[
      ''
       export HOME=$(pwd)
       rustup default stable
       rustup target add wasm32-wasip1
        cargo component''
          (nixpkgs.lib.strings.removePrefix "cargo" command)
        ];
      # ''
      #  export HOME=$(pwd)
      #  rustup default stable
      #  rustup target add wasm32-wasip1
      #   cargo  $cargo_options \
      #     component build \
      #     --target wasm32-unknown-unknown \
      #     $cargo_build_options >> $cargo_build_output_json
      # '';
      # CARGO_COMPONENT_CACHE_DIR = "$out";
    };

    devShells.aarch64-darwin.default = pkgs.mkShell {
      packages = deps;
    };

  };
}
