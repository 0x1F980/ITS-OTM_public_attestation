# shell.nix - Hermetic Dev Shell for ITS-OTM_public_attestation
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "its-otm-build-env";
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    musl
  ];

  shellHook = ''
    export RUSTFLAGS="-C target-feature=+aes,+ssse3 -C link-arg=-s"
    echo "Hermetic ITS-OTM Public Attestation Nix Environment Loaded!"
  '';
}
