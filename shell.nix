let
    pkgs = import <nixpkgs> {};
in 
    pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
        buildInputs = with pkgs; [
            rustup
            rust-analyzer

	    gcc
	    pkg-config
        ];

	shellHook = ''
		LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
#			pkgs.alsa-lib
		]}"
	'';
    }
