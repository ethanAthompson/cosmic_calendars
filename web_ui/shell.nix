{ pkgs ? import <nixpkgs> {}
}: pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo
    rustc
    rustup
    cargo-watch
    rustfmt
    lsof
  ];
}

# Development Guide 
# https://matthewrhone.dev/nixos-vscode-environment
# Run nix-shell
# cd src && leptosfmt *

# You can nest nix-shells wow..

# P2et3 => lsof -t -i:8000

#  udo kill -9 <PID>
# where <PID> is the result from [P2et3]