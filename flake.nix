{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, rust-overlay, nixpkgs, }: let
    pkgs = import nixpkgs {
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
    }; in {
    devShell.x86_64-linux = pkgs.mkShell {
      buildInputs = [
        (pkgs.rust-bin.stable.latest.default.override {
          targets = ["wasm32-unknown-unknown"];
        })
        pkgs.trunk
      ];
      RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
    };
  };
}
