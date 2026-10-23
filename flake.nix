{
  description = "Mini lang flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      systems = nixpkgs.lib.systems.flakeExposed;
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          mini_lang = pkgs.callPackage ./nix/package.nix { };
        in
        {
          inherit mini_lang;
          default = mini_lang;
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          rustdoc-std = pkgs.writeShellScriptBin "rustdoc-std" ''
            page="''${1:-std/index.html}"
            ''${BROWSER:-xdg-open} "${pkgs.rustc.unwrapped.doc}/share/doc/docs/html/$page" >/dev/null 2>&1 &
          '';
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              nixd
              nixfmt
              cargo
              rustc
              rustfmt
              clippy
              rust-analyzer
              rustdoc-std
            ];

            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };
        }
      );
    };
}
