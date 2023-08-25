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
  files = ./files;
  src = builtins.filterSource (path: type: type != "directory" || builtins.baseNameOf path != "target") ./.;
  server = naersk.buildPackage { inherit src; remapPathPrefix = true; };
  server_path = server.outPath;
in
pkgs.stdenv.mkDerivation rec {
  name = "server-with-files";
  buildInputs = [files server];
  inherit server_path;
  builder = builtins.toFile "builder.sh" ''
      source $stdenv/setup
      mkdir $out
      mkdir $out/bin
      cp -r $server_path/bin/* $out/bin/.
      mkdir $out/files
      echo "Copying files"
      cp -r ${files}/* $out/files/.
    '';
  }
