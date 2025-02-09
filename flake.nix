{
  description = "A simple, dynamically typed, interpreted language. It is
  designed to be easy to learn and use. The Nana interpreter runs as a
  WebAssembly component.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    pkgs = nixpkgs.legacyPackages.aarch64-darwin;
    deps = [
      # For build
      pkgs.rustup
      pkgs.cargo
      pkgs.wasmtime
      pkgs.wasm-tools

      # For tests
      pkgs.fswatch

      # For project metrics
      pkgs.cloc
      pkgs.toybox
    ];
  in
  {
    devShells.aarch64-darwin.default = pkgs.mkShell {
      packages = deps;
    };

  };
}
