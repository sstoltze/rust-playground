# Build a tarball with docker info in './result' with 'nix-build docker.nix'
# Load into docker with 'docker load -i result'
# Run with 'docker run --rm -itp 52340:5000 sstoltze/server'
# The server should start on localhost:52340
{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs {};
  server = import ./server.nix { inherit sources pkgs; };
  name = "sstoltze/server";
  tag = "latest";
in
pkgs.dockerTools.buildLayeredImage {
  inherit name tag;
  contents = [ server ];

  config = {
    Cmd = [ "/bin/server" ];
    Env = [ "ROCKET_PORT=5000" ];
    WorkingDir = "/";
  };
}
