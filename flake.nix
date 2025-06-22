{
    description = "A very basic rust dev env flake";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    };

    outputs = { self , nixpkgs }:
    let
        rust-overlay = (import (builtins.fetchTarball {
            url = "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
            sha256 = "05xyk469bj6zkvkk4gmc58rkiyavamn4xhfglwkdqlanqiyfwdfz";
        }));
        pkgs = (import nixpkgs {
                system = "x86_64-linux";
                overlays = [ rust-overlay ];
        });

        # pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in
    {
        devShells."x86_64-linux".default = pkgs.mkShell {
            buildInputs = [
                (pkgs.rust-bin.stable.latest.default.override {
                    extensions = ["rust-src"];
                })
                pkgs.cargo
                pkgs.rustup
                pkgs.rustfmt
            ];
            shellHook = ''
                rustup component add rust-analyzer
            '';
        };
    };
}

