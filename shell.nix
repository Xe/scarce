let
  sources = import ./nix/sources.nix;
  moz_overlay = import sources.nixpkgs-mozilla;
  pkgs = import sources.nixpkgs { overlays = [ moz_overlay ]; };
in pkgs.mkShell {
  buildInputs = with pkgs; [ latest.rustChannels.stable.rust bashInteractive openssl pkg-config ];
}
