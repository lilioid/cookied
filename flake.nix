{
  description = "cookied";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; };
    in rec {
      # packaging
      packages.x86_64-linux = rec {
        default = cookied;
        cookied = pkgs.callPackage ./nix/package.nix {};
      };
  
      overlays.default = final: prev: {
        cookied = packages.x86_64-linux.cookied;
      };

      nixosModules = rec {
        default = cookied;
        cookied = import ./nix/module.nix;
      };

      # development utilities
      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = with pkgs; [
          rustup
          systemfd
          watchexec
        ];
      };
      formatter.x86_64-linux = pkgs.nixfmt-rfc-style;
  };
}
