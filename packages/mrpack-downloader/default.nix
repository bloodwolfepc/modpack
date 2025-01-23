{ pkgs, lib, ... }:
  pkgs.rustPlatform.buildRustPackage rec {
    pname = "mrpack-downloader";
    version = "0.4.0"; 

    src = pkgs.fetchFromGitHub {
      owner = "JohnTheCoolingFan";
      repo = pname;
      tag = "v0.4.0";
      hash = "sha256-6AnBtC3YLFEIG89tlhkKBa35oJwzl883zAHZmuoqyW0=";

    };
      buildInputs = with pkgs; [ openssl ];
      nativeBuildInputs = with pkgs; [
        cargo 
        rustc
        pkg-config
      ];
  cargoHash = "sha256-hCtevOiIQ9QKTZMBUjJmmjYHmWl8Ghn7Mdil9IheF58=";

  meta = {
    description = "Download Modrinth Modpacks from mrpack files.";
    homepage = "https://github.com/JohnTheCoolingFan/mrpack-downloader";
    license = lib.licenses.mit;
    maintainers = [ "bloodwolfepc" ];
  };
}
