{ sources ? import ./nix/sources.nix,
  pkgs ? import sources.nixpkgs {} }:

let
  # import rust compiler
  rust = import ./nix/rust.nix { inherit sources; };
  # naersk is a nix tool for build rust crates
  naersk = pkgs.callPackage sources.naersk {
    rustc = rust;
    cargo = rust;
  };

  src = builtins.filterSource (path: type: type != "directory" || builtins.baseNameOf path != "target") ./.;
in
  naersk.buildPackage { inherit src; remapPathPrefix = true; }
