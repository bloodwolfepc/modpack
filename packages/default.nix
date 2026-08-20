{ pkgs ? import <nixpkgs> }: {
  mc = pkgs.callPackage ./mc { };
  mrpack-downloader = pkgs.callPackage ./mrpack-downloader { };
}
  
  

