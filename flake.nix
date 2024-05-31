{
    description = "Rust development environment";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
        flake-utils.url = "github:numtide/flake-utils";
        rust-overlay.url = "github:oxalica/rust-overlay";
        nixpkgs-esp-dev.url = "github:mirrexagon/nixpkgs-esp-dev";
        esp-dev.url = "github:thiskappaisgrey/nixpkgs-esp-dev-rust";
    };

    outputs = inputs@{ self, nixpkgs, flake-utils, rust-overlay, nixpkgs-esp-dev, esp-dev, ... }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system: 
        let
            pkgs = import nixpkgs {
                inherit system;
                overlays = [
                    rust-overlay.overlays.default
                    nixpkgs-esp-dev.overlays.default
                    (final:  prev:  
                        let
                            fhsEnv = prev.buildFHSUserEnv {
                                name = "esp32c3-toolchain-env";
                                targetPkgs = prev: with prev; [ zlib ];
                                runScript = "";
                            };
                        in {
                        gcc-riscv32-esp32c3-elf-bin = prev.stdenv.mkDerivation rec {
                            name = "esp32c3-toolchain";

                            src = prev.fetchurl {
                                url = "https://github.com/espressif/crosstool-NG/releases/download/esp-12.2.0_20230208/riscv32-esp-elf-12.2.0_20230208-x86_64-linux-gnu.tar.gz";
                                hash = "sha256-HrDWWZBUfulwa5BAZgDLw2OIFNX+t8H3tEu1QWR4pb0=";
                            };

                            buildInputs = [ prev.makeWrapper ];

                            phases = [ "unpackPhase" "installPhase" ];

                            installPhase = ''
                            cp -r . $out
                            for FILE in $(ls $out/bin); do
                              FILE_PATH="$out/bin/$FILE"
                              if [[ -x $FILE_PATH ]]; then
                                mv $FILE_PATH $FILE_PATH-unwrapped
                                makeWrapper ${fhsEnv}/bin/esp32c3-toolchain-env $FILE_PATH --add-flags "$FILE_PATH-unwrapped"
                              fi
                            done
                            '';

                        };}
                    )
                ];
            };
            rust-tar = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in rec {
            # devShell = (pkgs.buildFHSUserEnv {
            devShell = pkgs.mkShell {
                #name = "rust env";
                #targetPkgs = pkgs: (with pkgs; [ 
                buildInputs = with pkgs; [

                    cmake
                    rust-analyzer
                    rust-tar
                    pkg-config
                    openssl.dev

                    # Cargo tools
                    cargo-espflash
                    cargo-generate

                    # ESP-IDF requirements
                    ldproxy
                    flex
                    bison
                    gperf
                    ninja
                    ccache
                    libffi
                    libusb
                    dfu-util
                    python3Packages.pip
                    python3Packages.virtualenv
                    python3
                    gcc-riscv32-esp32c3-elf-bin
                #]);
                ];

            #    runScript = "bash";
            #}).env;
            };
        });
}
