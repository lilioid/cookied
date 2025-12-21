{ flake, nixpkgs, pkgs }: nixpkgs.lib.nixosSystem {
  specialArgs = flake.inputs;
  modules = [
    flake.outputs.nixosModules.default
  ] ++ [
    { nixpkgs.pkgs = pkgs; }
    ({ config, lib, pkgs, ... }: {

      # configure the VM itself
      system.stateVersion = config.system.nixos.release;
      system.nixos.label = "cookied-${lib.lists.last (lib.splitString "-" pkgs.cookied.name)}";
      system.nixos.variant_id = null;
      system.nixos.variantName = null;
      services.qemuGuest.enable = true;
      nix.enable = false;
      networking = {
        hostName = "cookied";
        useDHCP = true;
        tempAddresses = "disabled";
        nftables.enable = true;
      };

      # enable the actual cookied service
      services.cookied = {
        enable = true;
        algorithm = "text";
        text = "You have found our EasterEgg! Congrats! Come to the NOC to get a free snack and/or drink <3";
      };

    })
  ];
}
