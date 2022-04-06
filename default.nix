let
  holonixPath = builtins.fetchTarball "https://github.com/holochain/holonix/archive/e38f79f38dd0fb360821dc916c619252da03db77.tar.gz";
  holonix = import (holonixPath) {
    holochainVersionId = "v0_0_132";
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  packages = with nixpkgs; [
    # Additional packages go here
    nodejs-16_x
  ];
}
