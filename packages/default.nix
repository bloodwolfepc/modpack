{ pkgs ? import <nixpkgs> }: {
  mrpack-downloader = pkgs.callPackage ./mrpack-downloader { };
}
  
  

