let
    pkgs = import <nixpkgs> {};
in 
    pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
        buildInputs = with pkgs; [
            rustup
            rust-analyzer
        ];
    }
