{
  description = "A very basic flake";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/release-23.05";
  inputs.rust-overlay = {
    url = "github:oxalica/rust-overlay";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }:
  let
    pkgs = import nixpkgs {
      system = "x86_64-linux";
      overlays = [(import rust-overlay)];
    };

    toolchain = (
      pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml
    );


  in {
    devShells."x86_64-linux".default = pkgs.mkShell {
      name = "rust";
      buildInputs = [
        pkgs.openssl
        pkgs.pkg-config
        pkgs.rust-analyzer
        toolchain
        # (pkgs.writeScriptBin "wokwi-cli" ''
        #   #!${pkgs.stdenv.shell}
        #   exec ${pkgs.stdenv.shell} -c "${wokwi-cli}" --verbose
        # '')
      ];
      shellHook = ''
        export PS1="\n\[\033[1;34m\][\[\e]0;\u@rust: \w\a\]\u@rust:\w]\$\[\033[0m\] "

        # if [ ! -f ./rust-toolchain.toml ]; then
        #   echo "Creating rust-toolchain.toml..."
        #   cat <<EOF > ./rust-toolchain.toml
        #   [toolchain]
        #   channel = "stable"
        #   EOF
        # fi

       
      '';
    };
  };
}
