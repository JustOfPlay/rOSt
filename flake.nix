{
  description = "<name>";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

      app = pkgs.rustPlatform.buildRustPackage {
        pname = "app";
        version = "0.1.0";

        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        cargoBuildFlags = [
          "-p"
          "app"
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
          gtk3
          glib
          pango
          cairo
          gdk-pixbuf
          webkitgtk_4_1
          libsoup_3
          xdotool
        ];

        RUST_BACKTRACE = "1";
      };

      app-windows = pkgs.pkgsCross.mingwW64.rustPlatform.buildRustPackage {
        pname = "app";
        version = "0.1.0";

        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        cargoBuildFlags = [
          "-p"
          "app"
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
      };

    in
    {
      packages.${system} = {
        default = app;
        app = app;
        app-windows = app-windows;
      };

      apps.${system} = {
        default = {
          type = "app";
          program = "${app}/bin/app";
        };

        app = {
          type = "app";
          program = "${app}/bin/app";
        };
      };

      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          cargo
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
          gtk3
          glib
          pango
          cairo
          gdk-pixbuf
          webkitgtk_4_1
          libsoup_3
          xdotool
        ];

        RUST_BACKTRACE = "1";
      };
    };
}
