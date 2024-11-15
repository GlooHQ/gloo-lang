{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk/master";
    fenix.url = "github:nix-community/fenix";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, naersk, fenix, ... }:

    let

      buildTargets = {
        "x86_64-linux" = {
          crossSystemConfig = "x86_64-unknown-linux-musl";
          rustTarget = "x86_64-unknown-linux-musl";
        };
        "aarch64-linux" = {
          crossSystemConfig = "x86_64-unknown-linux-musl";
          rustTarget = "x86_64-unknown-linux-musl";
        };
        "aarch64-darwin" = {};
        "wasm" = {
          crossSystemConfig = "wasm32-unknown-unknown";
          rustTarget = "wasm32-unknown-unknown";
          makeBuildPackageAttrs = pkgsCross: {
            OPENSSL_STATIC = null;
            OPENSSL_LIB_DIR = null;
            OPENSSL_INCLUDE_DIR = null;
            CARGO_BUILD_RUSTFLAGS = [
              "-C" "linker=${pkgsCross.stdenv.cc}/bin/${}"
            ];
          };
        };

      };

      mkPkgs = buildSystem: targetSystem: import nixpkgs ({
        system = buildSystem;
      } // (if targetSystem == null then {} else {
        crossSystemcnofig = buildTargets.${targetSystem}.crossSystemConfig;
      }));

      eachSystem = supportedSystems: callback: builtins.fold'
        (overall: system: overall // { ${system} = callback system; })
        {}
        supportedSystems;

      eachCrosssystem

    in

    flake-utils.lib.eachDefaultSystem (system:

      let
        pkgs = nixpkgs.legacyPackages.${system};
        version = (builtins.fromTOML (builtins.readFile ./engine/Cargo.toml)).workspace.package.version;

        appleDeps = with pkgs.darwin.apple_sdk.frameworks; [
          CoreServices
          SystemConfiguration
          pkgs.libiconv-darwin
        ];


      in
        {
          packages.default = pkgs.rustPlatform.buildRustPackage {
            pname = "baml-cli";
            version = version;
            src = ./engine;
            LIBCLANG_PATH = pkgs.libclang.lib + "/lib/";
            BINDGEN_EXTRA_CLANG_ARGS = if pkgs.stdenv.isDarwin then
              "-I${pkgs.llvmPackages_18.libclang.lib}/lib/clang/18/headers "
            else
              "-isystem ${pkgs.llvmPackages_18.libclang.lib}/lib/clang/18/include -isystem ${pkgs.glibc.dev}/include";

            # Modify the test phase to only run library tests
            checkPhase = ''
              runHook preCheck
              echo "Running cargo test --lib"
              cargo test --lib
              runHook postCheck
            '';

            buildInputs = [pkgs.openssl pkgs.pkg-config] ++ (if pkgs.stdenv.isDarwin then appleDeps else []);
            nativeBuildInputs = [
              pkgs.openssl
              pkgs.pkg-config
              pkgs.ruby
            ];
            cargoLock = {
              lockFile = ./engine/Cargo.lock;
              outputHashes = {
                "pyo3-asyncio-0.21.0" = "sha256-5ZLzWkxp3e2u0B4+/JJTwO9SYKhtmBpMBiyIsTCW5Zw=";
                "serde_magnus-0.9.0" = "sha256-+iIHleftJ+Yl9QHEBVI91NOhBw9qtUZfgooHKoyY1w4=";
              };
            };
          };
        }
    );
}
