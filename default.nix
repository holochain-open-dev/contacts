let
  holonixPath = builtins.fetchTarball "https://github.com/holochain/holonix/archive/9c9a5a00dc05b0825841fae4ff8181182d9949ce.tar.gz";
  holonix = import (holonixPath) {
    holochainVersionId = "v0_0_122";
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  packages = [
    # additional packages go here
  ];
}
