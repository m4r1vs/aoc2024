{
  description = "Dev setup for Advent of Code 2024";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.05";
  };
  outputs = {nixpkgs, ...}: let
    lib = nixpkgs.lib;
    supportedSystems = [
      "x86_64-linux"
      "i686-linux"
      "aarch64-linux"
      "riscv64-linux"
      "aarch64-darwin"
    ];
    forAllSystems = lib.genAttrs supportedSystems;
  in {
    devShell = forAllSystems (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
        with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustc
            ];
          }
    );
  };
}
