{ lib, rustPlatform }: let
  cargoToml = lib.importTOML ../Cargo.toml;
in rustPlatform.buildRustPackage {
  pname = cargoToml.package.name;
  version = cargoToml.package.version;

  src = ../.;
  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  meta = with lib; {
    description = cargoToml.package.description;
    homepage = cargoToml.package.repository;
    license = licenses.mit;
    maintainers = [ maintainer.lilioid ];
  };
}
