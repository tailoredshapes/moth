{
  pkgs ? import <nixpkgs> {},
}:

pkgs.mkShellNoCC {

  buildInputs = [
        pkgs.rustc
        pkgs.cargo
        pkgs.rustfmt
        pkgs.clippy
    ];
   
}
