let sources = import ./nix/sources.nix;
    rust = import ./nix/rust.nix { inherit sources; };
    pkgs = import sources.nixpkgs { };
    in
  pkgs.mkShell {
    buildInputs = [
      (rust.override { extensions = ["rust-src"]; })
    ];
    nativeBuildInputs = [ pkgs.nasm
                        ];
}
