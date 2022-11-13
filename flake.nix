{
  description = "Raspberry Pi Pico (RP2040) support for Drone, an Embedded Operating System.";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-22.05";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pico-sdk = {
      url = "github:raspberrypi/pico-sdk/1.4.0";
      flake = false;
    };
  };

  outputs = { self, utils, nixpkgs, fenix, pico-sdk }:
    utils.lib.eachDefaultSystem (system:
      let
        buildTarget = "thumbv6m-none-eabi";
        rustFlags = ''--cfg drone_cortexm="cortexm0plus_r0p1"'';
        rustChannel = {
          channel = "nightly";
          date = "2022-11-12";
          sha256 = "NZrKSshDgITZuDSffP89NpZl/pQlblc7arXatkV+O9A=";
        };

        pkgs = nixpkgs.legacyPackages.${system};
        rustToolchain = with fenix.packages.${system}; combine
          ((with toolchainOf rustChannel; [
            rustc
            cargo
            clippy
            rustfmt
            rust-src
          ]) ++ (with targets.${buildTarget}.toolchainOf rustChannel; [
            rust-std
          ]));
        rustAnalyzer = fenix.packages.${system}.rust-analyzer;

        crossEnv = {
          CARGO_BUILD_TARGET = buildTarget;
        };
        nativeEnv = {
          CARGO_BUILD_TARGET = pkgs.stdenv.targetPlatform.config;
        };

        cargoRdme = (
          pkgs.rustPlatform.buildRustPackage rec {
            name = "cargo-rdme";
            src = pkgs.fetchFromGitHub {
              owner = "orium";
              repo = name;
              rev = "v0.7.3";
              sha256 = "qzit/uYkyWiOqpO5sHYo2hKJvOhovcO+oVbq/Bo2HsI=";
            };
            cargoSha256 = "lbyLVmSLNt4mt6hQbJnCuNL1Y1/2E/81sVpLYOkv7w8=";
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl ];
            doCheck = false;
          });

        checkAll = pkgs.writeShellScriptBin "check-all" ''
          set -ex
          cargo rdme --check
          cargo fmt --all --check
          find sdk -name '*.[ch]' | xargs clang-format --dry-run -Werror
          cargo clippy --workspace --exclude drone-raspberrypi-pico-gen --features all -- --deny warnings
          nix develop '.#native' -c cargo clippy --package drone-raspberrypi-pico-gen -- --deny warnings
          nix develop '.#native' -c cargo test --workspace --features all,host
          RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --package drone-raspberrypi-pico --features all
          RUSTDOCFLAGS='-D warnings' nix develop '.#native' -c cargo doc --no-deps --package drone-raspberrypi-pico-gen
        '';

        clangFormatAll = pkgs.writeShellScriptBin "clang-format-all" ''
          find sdk -name '*.[ch]' | xargs clang-format -i
        '';

        generateMapPieces = pkgs.writeShellScriptBin "generate-map-pieces" ''
          for i in $(seq 2 $1); do
            rm -rf src/map/pieces/$i
            cp -r src/map/pieces/1 src/map/pieces/$i
            sed -i "s/\(pieces[-_]\)1\b/\1$i/g" src/map/pieces/$i/Cargo.toml
            sed -i "s/\(generate_regs(\)1\(,\s\+[0-9]\+)\)/\1$i\2/" src/map/pieces/$i/build.rs
          done
        '';

        updateVersions = pkgs.writeShellScriptBin "update-versions" ''
          sed -i "s/\(api\.drone-os\.com\/drone-raspberrypi-pico\/\)[0-9]\+\(\.[0-9]\+\)\+/\1$(echo $1 | sed 's/\(.*\)\.[0-9]\+/\1/')/" \
            Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml src/lib.rs
          sed -i "/\[.*\]/h;/version = \".*\"/{x;s/\[workspace.package\]/version = \"$1\"/;t;x}" \
            Cargo.toml
          sed -i "/\[.*\]/h;/version = \"=.*\"/{x;s/\[.*drone-raspberrypi-pico-.*\]/version = \"=$1\"/;t;x}" \
            Cargo.toml
          sed -i "/\[.*\]/h;/version = \".*\"/{x;s/\[.*drone-config\]/version = \"$2\"/;t;x}" \
            Cargo.toml
          sed -i "/\[.*\]/h;/version = \".*\"/{x;s/\[.*drone-core\]/version = \"$3\"/;t;x}" \
            Cargo.toml
          sed -i "/\[.*\]/h;/version = \".*\"/{x;s/\[.*drone-cortexm\]/version = \"$4\"/;t;x}" \
            Cargo.toml
          sed -i "/\[.*\]/h;/version = \".*\"/{x;s/\[.*drone-gen\]/version = \"$5\"/;t;x}" \
            Cargo.toml
          sed -i "s/\(drone-raspberrypi-pico.*\)version = \"[^\"]\+\"/\1version = \"$1\"/" \
            src/lib.rs
        '';

        publishCrates = pkgs.writeShellScriptBin "publish-crates" ''
          cd gen && nix develop '.#native' -c cargo publish
          cd sdk && cargo publish
          cd src/pieces/traits && cargo publish
          sleep 30
          cd src/pieces/1 && cargo publish
          cd src/pieces/2 && cargo publish
          cd src/pieces/3 && cargo publish
          cd src/pieces/4 && cargo publish
          cd src/pieces/5 && cargo publish
          cd src/pieces/6 && cargo publish
          cd src/pieces/7 && cargo publish
          cd src/pieces/8 && cargo publish
          cd src/pieces/9 && cargo publish
          cd src/pieces/10 && cargo publish
          cd src/pieces/11 && cargo publish
          cd src/pieces/12 && cargo publish
          sleep 30
          cd src/pieces && cargo publish
          sleep 30
          cd src/periph/clock && cargo publish
          cd src/periph/gpio && cargo publish
          cd src/periph/pll && cargo publish
          sleep 30
          cargo publish --features all
        '';

        publishDocs = pkgs.writeShellScriptBin "publish-docs" ''
          dir=$(sed -n 's/.*api\.drone-os\.com\/\(.*\/.*\)\/.*\/"/\1/;T;p' Cargo.toml) \
            && rm -rf ../drone-api/$dir \
            && cp -rT target/doc ../drone-api/$dir \
            && cp -rT target/$CARGO_BUILD_TARGET/doc ../drone-api/$dir \
            && echo '<!DOCTYPE html><meta http-equiv="refresh" content="0; URL=./drone_raspberrypi_pico">' > ../drone-api/$dir/index.html \
            && cd ../drone-api && git add $dir && git commit -m "Docs for $dir"
        '';

        mkShell = extraEnv: pkgs.mkShell ({
          nativeBuildInputs = [
            rustToolchain
            rustAnalyzer
            cargoRdme
            checkAll
            clangFormatAll
            generateMapPieces
            updateVersions
            publishCrates
            publishDocs
          ] ++ (with pkgs; [
            cmake
            python3
          ]) ++ (with pkgs.pkgsCross.arm-embedded; [
            stdenv.cc
            libcCross
          ]);
          PICO_SDK_PATH = pico-sdk;
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          CARGO_BUILD_RUSTFLAGS = rustFlags;
          EXTRA_CLANG_CFLAGS = with pkgs.pkgsCross.arm-embedded.stdenv;
            builtins.toString ([ "-nostdinc" ] ++ builtins.map (path: "-isystem ${path}") [
              "${cc.cc}/lib/gcc/${targetPlatform.config}/${cc.cc.version}/include"
              "${cc.cc}/lib/gcc/${targetPlatform.config}/${cc.cc.version}/include-fixed"
              "${cc.cc}/${targetPlatform.config}/sys-include"
            ]);
        } // extraEnv);
      in
      {
        devShells = rec {
          cross = mkShell (crossEnv // { name = "cross"; });
          native = mkShell (nativeEnv // { name = "native"; });
          default = cross;
        };
      }
    );
}
