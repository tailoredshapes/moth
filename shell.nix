{
  pkgs ? import <nixpkgs> {},
}:

let
  moth = (builtins.getFlake "github:tailoredshapes/moth")
      .packages.${pkgs.system}.default;

in pkgs.mkShellNoCC {

  buildInputs = [
        pkgs.rustc
        pkgs.cargo
        pkgs.rustfmt
        pkgs.clippy
	moth
    ];
   
}
