{ pkgs, customPkgs }:

pkgs.mkShell {
  shellHook = ''
    export DEVSHELL=1
  '';

  nativeBuildInputs =
    with pkgs;
    [
      packwiz
      unzip
      zip
      yq
      portablemc
    ]
    ++ [
      customPkgs."mrpack-downloader"
    ];
}
