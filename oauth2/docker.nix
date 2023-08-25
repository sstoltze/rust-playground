# Build a tarball with docker info in './result' with 'nix-build docker.nix'
# Load into docker with 'docker load -i result'
# Run with 'docker run --rm -itp 52340:5000 sstoltze/server'
# The server should start on localhost:52340
{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs {};
  server = import ./oauth2-server.nix { inherit sources pkgs; };
  name = "sstoltze/server";
  tag = "latest";
  port = "5000";
in
pkgs.dockerTools.buildLayeredImage {
  inherit name tag;
  contents = [ server ];

  config = {
    Cmd = [ "/bin/executable-name" ];
    Env = [ "ROCKET_PORT=${port}" ];
    WorkingDir = "/";
    ExposedPorts = {
      "${port}" = { };
    };
  };

}
