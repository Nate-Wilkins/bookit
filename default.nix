{
  gitignore,
  pkgs,
  lib,
  manifest,
  toolchain,
  dependencies
}: (
  let
    getSrc                                        = src:
      let
        isGitIncluded = gitignore { basePath = src; extraRules = ''
          *
          !shell/
          !src/
          !tests/
          !.envrc.sh
          !Cargo.lock
          !Cargo.toml
          !default.nix
          !flake.nix
          !LICENSE
        ''; };
      in
        lib.cleanSourceWith {
          inherit src;
          name = "source";
          filter = isGitIncluded;
        };

    name                                          = manifest.name;
    version                                       = manifest.version;
  in (
    ((pkgs.makeRustPlatform {
      cargo                                       = toolchain;
      rustc                                       = toolchain;
    }).buildRustPackage {
      buildType                                   = "release";
      pname                                       = name;
      version                                     = version;
      src                                         = getSrc ./.;

      nativeBuildInputs                           = with pkgs; [
        makeWrapper
        pkg-config
      ] ++ dependencies;
      cargoSha256                                 = "sha256-0hfmV4mbr3l86m0X7EMYTOu/b+BjueVEbbyQz0KgOFY=";
      cargoLock.lockFile                          = ./Cargo.lock;
      meta                                        = { };

      doCheck                                     = false; # Enabling is Impure since postFixup isn't accounted for.
      postFixup                                   = ''
        wrapProgram "$out/bin/${name}" \
          --prefix PATH : ${lib.makeBinPath dependencies}
      '';
    })
  )
)
