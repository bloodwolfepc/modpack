{ pkgs, ... }: {
  pkgs.rustPlatform.buildRustPackage = {
    pname = "mc";
    version = "0.1";
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;
  };
}
