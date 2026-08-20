{
  description = "Modpack by bloodwolfe";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        customPkgs = import ./packages { inherit pkgs; };
      in
      {
        packages = customPkgs;

        devShells.default = import ./devshell.nix {
          inherit pkgs customPkgs;
        };
      }
    );
}
