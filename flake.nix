{
  description = "Modpack by bloodwolfe";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system: let 
        pkgs = import nixpkgs { inherit system; };
        pkgs' = import ./packages { inherit pkgs; };
        shellHook = ''
          alias pw="packwiz"
          alias pl="prismlauncher"
          alias mc="portablemc"
          alias mc-logs="cat ./instances/blood-mc-0.3.0/.minecraft/logs/latest.log"
        '';
        nativeBuildInputs = [
          pkgs.packwiz
          pkgs.unzip
          pkgs.zip
          pkgs.yq
          pkgs.portablemc
          pkgs.hello
          pkgs'.mrpack-downloader
        ];
          #firium 
          #nur.repos.ihaveamac.pkgs.mrpack-install
          #packsquash
      in {
        inherit pkgs';
        devShells = rec {
          modpack = pkgs.mkShell {
            inherit nativeBuildInputs shellHook;
          };
          default = modpack;
        };
      });
}
/*
portablemc \
--main-dir ./instances/blood-mc-0.3.0/.minecraft \
-v \
start fabric:1.20.1 \
--dry



*/
