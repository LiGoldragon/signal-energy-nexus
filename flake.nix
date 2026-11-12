{
  description = "signal for the Energy Nexus";
  inputs = {
    tests = { url = "github:LiGoldragon/energy-nexus-tests/9c233904b6af009aef96139bae4aee6a6fbbafdc"; flake = false; };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-build = { url = "github:LiGoldragon/rust-build"; inputs.nixpkgs.follows = "nixpkgs"; };
  };
  outputs = { nixpkgs, flake-utils, rust-build, tests, ... }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs { inherit system; };
      rust = rust-build.lib.${system}.fromPkgs pkgs;
      source = pkgs.runCommand "signal-energy-nexus-test-source" {} ''
        cp -R ${./.}/. "$out"
        chmod -R u+w "$out"
        mkdir -p "$out/tests"
        cp ${tests}/scenarios/generated-payloads/ordinary.rs "$out/tests/stage-one.rs"
      '';
      cargoArtifacts = rust.craneLib.buildDepsOnly { src = source; };
    in {
      checks.test = rust.craneLib.cargoTest { src = source; inherit cargoArtifacts; cargoTestExtraArgs = "--all-features"; };
      checks.plain = rust.craneLib.cargoTest { src = source; inherit cargoArtifacts; cargoTestExtraArgs = "--no-default-features"; };
    });
}
