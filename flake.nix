{
  description = "Swarm Protocol development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:

    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustup
            git-cliff
          ];

          shellHook = ''
            PROJECT_NAME=$(basename "$PWD")
            DATA_DIR="''${XDG_DATA_HOME:-$HOME/.local/share}/rust-dev-envs/$PROJECT_NAME"
            SHELL_DIR="$DATA_DIR/shell"
            export CARGO_HOME="$DATA_DIR/cargo"
            export RUSTUP_HOME="$DATA_DIR/rustup"
            export PATH=$CARGO_HOME/bin:$PATH

            mkdir -p "$SHELL_DIR"
            mkdir -p "$CARGO_HOME"
            mkdir -p "$RUSTUP_HOME"

            rustup default stable
            rustup toolchain install
            rustup update

            cargo install cargo-edit
            cargo install bacon
            cargo install cargo-watch
            cargo install cargo-outdated

            echo
            echo
            echo

            echo "Rustup installed at $RUSTUP_HOME"
            echo "Cargo installed at $CARGO_HOME"
            echo "git-cliff installed at $(which git-cliff)"
            echo "cargo-edit installed at $(which cargo-add)"
            echo "bacon installed at $(which bacon)"
            echo "cargo-watch installed at $(which cargo-watch)"
            echo "cargo-outdated installed at $(which cargo-outdated)"
            echo
            echo "$(rustup --version)"
            echo "$(cargo --version)"
            echo "$(git-cliff --version)"
            echo "cargo-edit unknown, no --version flag available"
            echo "$(bacon --version)"
            echo "$(cargo-watch --version)"
            echo "cargo-outdated unknown, no --version flag available"
            echo
            echo "Happy coding! :)"
            echo
          '';
        };
      }
    );
}