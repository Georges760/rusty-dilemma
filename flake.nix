{
  description = "things";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    parts.url = "github:hercules-ci/flake-parts";
    parts.inputs.nixpkgs-lib.follows = "nixpkgs";
  };

  outputs =
    inputs@{ self
    , nixpkgs
    , crane
    , fenix
    , parts
    , ...
    }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = nixpkgs.lib.systems.flakeExposed;
      imports = [
      ];
      perSystem = { config, pkgs, system, lib, ... }:
        let
          arm-toolchain-plain = fenix.packages.${system}.fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "sha256-SKrgSoah/i97Qng3btr3l2ax9yn2eftY0+yofsPNkoY=";
          };
          native-toolchain = (fenix.packages.${system}.complete.withComponents [
            "cargo"
            "clippy"
            # "rust-src"
            # "rustc"
            # "rustfmt"
          ]);
          arm-toolchain = pkgs.runCommand "turbowaker-rust" { } ''
              echo "test $out ${arm-toolchain-plain}"
              cp -RL ${arm-toolchain-plain} $out
              chmod -R +rwx $out

              echo "doing patch"

              patch $out/lib/rustlib/src/rust/library/core/Cargo.toml ${./turbowaker/Cargo.toml.patch}
              patch $out/lib/rustlib/src/rust/library/core/src/task/wake.rs ${./turbowaker/wake.rs.patch}
            '';
          
          toolchain = fenix.packages.${system}.combine [ arm-toolchain native-toolchain ];
          craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

          src = craneLib.cleanCargoSource ./.;
          
          package = { target ? "thumbv6m-none-eabi", args ? "", profile ? "release" }: craneLib.buildPackage {
            inherit src;
          
            cargoVendorDir = craneLib.vendorMultipleCargoDeps {
              inherit (craneLib.findCargoFiles src) cargoConfigs;
              cargoLockList = [
                ./Cargo.lock

                # Unfortunately this approach requires IFD (import-from-derivation)
                # otherwise Nix will refuse to read the Cargo.lock from our toolchain
                # (unless we build with `--impure`).
                #
                # Another way around this is to manually copy the rustlib `Cargo.lock`
                # to the repo and import it with `./path/to/rustlib/Cargo.lock` which
                # will avoid IFD entirely but will require manually keeping the file
                # up to date!
                "${toolchain}/lib/rustlib/src/rust/Cargo.lock"
              ];
            };

            cargoExtraArgs = "-Z build-std=core,panic_abort,alloc -Z build-std-features=panic_immediate_abort,core/turbowakers --target ${target} ${args}";
            CARGO_PROFILE = profile;
            pname = "rusty-dilemma";
            version = "0.1.0";
            
            strictDeps = true;
            doCheck = false;
            buildInputs = [
              # Add additional build inputs here
            ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              # Additional darwin specific inputs can be set here
              pkgs.libiconv
            ];
          };
          elf = pkg: name: pkgs.runCommandLocal "mkelf" { } ''
            mkdir -p $out
            cp ${pkg}/bin/${name} $out/${name}.elf
          '';
          binary = pkg: name: pkgs.runCommandLocal "mkbinary" { buildInputs = [ pkgs.llvm ]; } ''
            mkdir -p $out
            llvm-objcopy -O binary ${pkg}/bin/${name} $out/${name}.bin
          '';
        in
        rec
        {
          devShells.default = craneLib.devShell {
            packages = with pkgs; [ picotool libiconv just ];
          };
          # binaries don't currently work because we need the macros package to be built with std supported, crane seems to do some weird stuff that makes it forced onto a no-std build env
          # devShells.default = pkgs.mkShell {
          #   # inputsFrom = [ (firmware { args = "--lib"; profile = "dev"; }) ];
          #   nativeBuildInputs = with pkgs; [
          #     # fenix.packages.${system}.rust-analyzer
          #     # cargo-binutils
          #     # probe-rs
          #     picotool
          #     # pkgsCross.arm-embedded.buildPackages.binutils
          #   ];
          # };
        #   packages.default = package { args = "--lib"; profile = "dev"; };
        #   packages.bin = package { args = "--bin binary --no-default-features"; profile = "release"; };
        #   packages.debug-bin = package { args = "--bin binary --features probe,m2"; profile = "dev"; };
        #   packages.binaries = pkgs.symlinkJoin {
        #     name = "dilemma-binaries";
        #     paths = [
        #       (elf packages.bin "binary")
        #       (binary packages.bin "binary")
        #     ];
        #   };
        #   packages.debug-binaries = pkgs.symlinkJoin {
        #     name = "dilemma-binaries";
        #     paths = [
        #       (elf packages.debug-bin "binary")
        #       (binary packages.debug-bin "binary")
        #     ];
        #   };
        };
    };
}
