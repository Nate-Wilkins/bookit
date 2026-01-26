# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
#
# Author:      Nate-Wilkins <nate-wilkins@code-null.com>
#
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
{
  description                                                     = "Fast and simple bookmark manager for your operating system.";

  inputs                                                          = {
    systems.url                                                   = "path:./flake.systems.nix";
    systems.flake                                                 = false;

    nixpkgs.url                                                   = "github:Nate-Wilkins/nixpkgs/nixos-unstable";

    flake-utils.url                                               = "github:numtide/flake-utils";
    flake-utils.inputs.systems.follows                            = "systems";

    gitignore.url                                                 = "github:hercules-ci/gitignore.nix";
    gitignore.inputs.nixpkgs.follows                              = "nixpkgs";

    fenix.url                                                     = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows                                  = "nixpkgs";
    fenix.inputs.rust-analyzer-src.follows                        = "rust-analyzer-src";

    asciinema-automation.url                                      = "github:Nate-Wilkins/asciinema-automation/3.0.0";
    asciinema-automation.inputs.systems.follows                   = "systems";
    asciinema-automation.inputs.nixpkgs.follows                   = "nixpkgs";
    asciinema-automation.inputs.flake-utils.follows               = "flake-utils";
    asciinema-automation.inputs.fenix.follows                     = "fenix";
    asciinema-automation.inputs.jikyuu.follows                    = "jikyuu";
    asciinema-automation.inputs.rust-analyzer-src.follows         = "rust-analyzer-src";
    asciinema-automation.inputs.task-documentation.follows        = "task-documentation";
    asciinema-automation.inputs.task-runner.follows               = "task-runner";

    task-documentation.url                                         = "gitlab:ox_os/task-documentation/5.1.0";
    task-documentation.inputs.systems.follows                      = "systems";
    task-documentation.inputs.nixpkgs.follows                      = "nixpkgs";
    task-documentation.inputs.flake-utils.follows                  = "flake-utils";
    task-documentation.inputs.fenix.follows                        = "fenix";
    task-documentation.inputs.asciinema-automation.follows         = "asciinema-automation";
    task-documentation.inputs.jikyuu.follows                       = "jikyuu";
    task-documentation.inputs.rust-analyzer-src.follows            = "rust-analyzer-src";
    task-documentation.inputs.task-runner.follows                  = "task-runner";

    task-runner.url                                                = "gitlab:ox_os/task-runner/4.0.2";
    task-runner.inputs.systems.follows                             = "systems";
    task-runner.inputs.nixpkgs.follows                             = "nixpkgs";
    task-runner.inputs.flake-utils.follows                         = "flake-utils";
    task-runner.inputs.fenix.follows                               = "fenix";
    task-runner.inputs.asciinema-automation.follows                = "asciinema-automation";
    task-runner.inputs.jikyuu.follows                              = "jikyuu";
    task-runner.inputs.rust-analyzer-src.follows                   = "rust-analyzer-src";
    task-runner.inputs.task-documentation.follows                  = "task-documentation";

# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
# Transatives
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
    jikyuu.url                                                     = "github:Nate-Wilkins/jikyuu/3.0.1";
    jikyuu.inputs.systems.follows                                  = "systems";
    jikyuu.inputs.nixpkgs.follows                                  = "nixpkgs";
    jikyuu.inputs.flake-utils.follows                              = "flake-utils";
    jikyuu.inputs.fenix.follows                                    = "fenix";
    jikyuu.inputs.asciinema-automation.follows                     = "asciinema-automation";
    jikyuu.inputs.rust-analyzer-src.follows                        = "rust-analyzer-src";
    jikyuu.inputs.task-documentation.follows                       = "task-documentation";
    jikyuu.inputs.task-runner.follows                              = "task-runner";

    rust-analyzer-src.url                                          = "github:rust-lang/rust-analyzer/nightly";
    rust-analyzer-src.flake                                        = false;
  };

  outputs                                                         = {
    nixpkgs,
    flake-utils,
    gitignore,
    fenix,
    asciinema-automation,
    task-runner,
    task-documentation,
    ...
  }: (
    let
      mkPkgs                                                       =
        system:
          pkgs: (
            # NixPkgs
            import pkgs { inherit system; }
            //
            # Custom Packages.
            {
              asciinema-automation                                 = asciinema-automation.defaultPackage."${system}";
              task-documentation                                   = task-documentation.defaultPackage."${system}";
            }
          );
    in (
      flake-utils.lib.eachDefaultSystem (system: (
        let
          pkgs                                                     = mkPkgs system nixpkgs;
          manifest                                                 = (pkgs.lib.importTOML ./Cargo.toml).package;
          name                                                     = manifest.name;
          environment                                              = {
            inherit pkgs;
            inherit manifest;
            toolchain                                              = fenix.packages.${system}.minimal.toolchain;
            gitignore                                              = gitignore.lib.gitignoreFilterWith;
            dependencies                                           = with pkgs; [
              gawk                 # /bin/awk
              util-linux           # /bin/lsblk      # /bin/mount  # /bin/umount
              coreutils            # /bin/mkdir      # /bin/tail   # /bin/head
            ];
          };
        in rec {
          packages.${name}                                         = pkgs.callPackage ./default.nix environment;
          legacyPackages                                           = packages;

          # `nix build`
          defaultPackage                                           = packages.${name};

          # `nix run`
          apps.${name}                                             = flake-utils.lib.mkApp {
            inherit name;
            drv                                                    = packages.${name};
          };
          defaultApp                                               = apps.${name};

          # `nix develop`
          devShells.default                                        = import ./shell/default.nix (
            environment
          // {
            taskRunner                                             = task-runner.taskRunner.${system};
          });
        })
      )
    )
  );
}