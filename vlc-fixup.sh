#run after initialisation to give waterframes the path to the VLC binary
#notworking
VLC_NIX_PATH=$(nix eval --raw --impure 'nixpkgs#vlc.outPath')
echo "$VLC_NIX_PATH/bin/vlc" > ./config/watermedia/custom_vlc_path.txt 
echo path: "$VLC_NIX_PATH/bin/vlc"
