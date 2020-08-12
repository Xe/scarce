let
  sources = import ./nix/sources.nix;
  moz_overlay = import sources.nixpkgs-mozilla;
  pkgs = import sources.nixpkgs { overlays = [ moz_overlay ]; };
  rust = pkgs.rustChannelOfTargets "stable" null [ "wasm32-unknown-unknown" ];
in pkgs.mkShell {
  buildInputs = with pkgs; [ rust bashInteractive openssl pkg-config ];

  RUST_LOG = "info";
}
