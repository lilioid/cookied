{
  description = "cookied";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; };
    in {
      # packaging
      overlays.default = import ./nix/overlay.nix;
      packages.x86_64-linux = rec {
        default = cookied;
        cookied = pkgs.callPackage ./nix/package.nix {};
      };

      nixosModules = rec {
        default = cookied;
        cookied = import ./nix/module.nix;
      };

      nixosConfigurations.cookied = import ./nix/vm.nix { flake = self; nixpkgs = nixpkgs; pkgs = pkgs; };

      # development utilities
      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = with pkgs; [
          systemfd
          watchexec
        ];
      };
      formatter.x86_64-linux = pkgs.nixfmt-rfc-style;
  };
}
