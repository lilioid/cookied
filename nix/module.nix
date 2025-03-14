{ lib, config, pkgs, ...}:
let
  cfg = config.services.cookied;
in {
  # api definition
  options = with lib.options; {
    services.cookied = {
      enable = mkEnableOption "cookied";
      package = mkPackageOption pkgs "cookied" {};
      openFirewall = mkOption {
        description = "Whether port 17 should be opened on the systems firewall";
        default = true;
      };
      algorithm = mkOption {
        description = "Which response algorithm to use";
        default = "time-and-place";
        type = lib.types.enum [ "pattern" "time-and-place" "text" ];
      };
      text = mkOption {
        description = "The text to use when algorithm == 'text'";
        default = "Hello World";
      };
    };
  };

  # implementation
  config = lib.mkIf cfg.enable {
    nixpkgs.overlays = [ (import ./overlay.nix) ];
  
    systemd.sockets."cookied" = {
      description = "RFC865 Quote of the Day Server";
      documentation = [ "https://codeberg.org/lilly/cookied" ];
      wantedBy = [ "multi-user.target" ];
      requiredBy = [ "cookied.service" ];
      listenStreams = [ "17" ];
      listenDatagrams = [ "17" ];
      socketConfig."BindIPv6Only" = "both";
    };

    systemd.services."cookied" = {
      description = "RFC865 Quote of the Day Server";
      documentation = [ "https://codeberg.org/lilly/cookied" ];
      serviceConfig.ExecStart = "${cfg.package}/bin/cookied --alg ${cfg.algorithm} --text '${cfg.text}'";
    };

    networking.firewall = lib.mkIf cfg.openFirewall {
      allowedUDPPorts = [ 17 ];
      allowedTCPPorts = [ 17 ];
    };
  };
}
