{
  inputs,
  cell,
}: let
  inherit (inputs) nixpkgs rust-overlay;
in rec {
  rust-bin =
    (nixpkgs.appendOverlays [
      (import rust-overlay)
    ])
    .rust-bin;
  rustToolchain = rust-bin.selectLatestNightlyWith (toolchain:
    toolchain.default.override {
      extensions = ["rustfmt" "rust-src" "miri"];
    });
}
