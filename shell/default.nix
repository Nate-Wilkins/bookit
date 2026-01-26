# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
#
# Author:      Nate-Wilkins <nate-wilkins@code-null.com>
#
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
{ pkgs, taskRunner, toolchain, dependencies, ... }: (
  let
    #
    # Run help.
    #
    task_help                                           = taskRunner.mkTask {
      name                                              = "help";
      dependencies                                      = with pkgs; [
        coreutils           # /bin/echo
      ];
      src                                               = with pkgs; ''
        ${coreutils}/bin/echo "                                                                     "
        ${coreutils}/bin/echo "Usage                                                                "
        ${coreutils}/bin/echo "   clean                  Run cleanup on temporary files.            "
        ${coreutils}/bin/echo "   build                  Run build for the project.                 "
        ${coreutils}/bin/echo "   test                   Run tests for the project.                 "
        ${coreutils}/bin/echo "   docs                   Run documentation generator.               "
        ${coreutils}/bin/echo "   show                   Run show info for the flake.               "
        ${coreutils}/bin/echo "   run                    Run the project.                           "
        ${coreutils}/bin/echo "   examples               Run generators for examples.               "
        ${coreutils}/bin/echo "   publish                Publish project binary.                    "
        ${coreutils}/bin/echo "   help                   Run help.                                  "
        ${coreutils}/bin/echo "                                                                     "
      '';
    };

    #
    # Run cleanup on ignored files.
    #
    task_clean                                                = taskRunner.mkTask {
      name                                                    = "clean";
      dependencies                                            = with pkgs; [
        coreutils           # /bin/echo
        findutils           # /bin/find  /bin/xargs
        git                 # /bin/git
      ];
      isolate                                                 = false;
      src                                                     = with pkgs; ''
        # Delete all ignored files.
        ${git}/bin/git ls-files -o --ignored --exclude-standard | ${findutils}/bin/xargs rm -rf

        # Delete all empty directories.
        ${findutils}/bin/find . -type d -empty -delete
      '';
    };

    #
    # Run build for the project.
    #
    task_build                                                = taskRunner.mkTask {
      name                                                    = "build";
      dependencies                                            = with pkgs; [
        imagemagick         # /bin/convert
        coreutils           # /bin/echo
        nix                 # /bin/nix
          git               # /bin/git
      ];
      isolate                                                 = false;
      src                                                     = with pkgs; ''
        ${nix}/bin/nix build \
          --experimental-features 'nix-command flakes' \
          --show-trace \
          --option max-jobs 8 \
          --option cores 2 \
          --verbose \
          --option eval-cache false \
          -L \
          "."

        # Generate Icon.
        ${imagemagick}/bin/convert \
          -background none \
          -density 1200 \
          -resize 128x128 \
          ./icon.svg \
          ./target/release/icon_128.png
      '';
    };

    #
    # Run tests for the project.
    #
    task_test                                                 = taskRunner.mkTask {
      name                                                    = "test";
      dependencies                                            = with pkgs; [
        cargo               # /bin/build      # /bin/teset
      ];
      isolate                                                 = false;
      src                                                     = with pkgs; ''
        ${cargo}/bin/cargo test
      '';
    };


    #
    # Run documentation generator.
    #
    task_docs                                                = taskRunner.mkTask {
      name                                                   = "docs";
      dependencies                                           = with pkgs; [
        coreutils           # /bin/echo  # /bin/mv
        task_examples       # /bin/examples
        task-documentation  # /bin/alex
      ];
      isolate                                                = false;
      src                                                    = with pkgs; ''
        ${task_examples}/bin/examples

        ${task-documentation}/bin/alex generate
        ${coreutils}/bin/mv ./docs/README.md ./README.md
      '';
    };

    #
    # Run show info for the flake.
    #
    task_show                                                 = taskRunner.mkTask {
      name                                                    = "show";
      dependencies                                            = with pkgs; [
        task_build          # /bin/run
        coreutils           # /bin/echo
        nix                 # /bin/nix  /bin/nix-store
          git               # /bin/git
        jq                  # /bin/jq
        graphviz            # /bin/graphviz
      ];
      isolate                                                 = false;
      src                                                     = with pkgs; ''
        # Build
        ${task_build}/bin/build

        # Flake
        ${nix}/bin/nix flake show \
          --experimental-features 'nix-command flakes'

        # Outputs
        ${nix}/bin/nix derivation show | ${jq}/bin/jq '.[].outputs'

        # Store
        TASK_SHOW_NIX_STORE_PATH=$(${nix}/bin/nix eval --inputs-from . --raw)
        ${coreutils}/bin/echo "$TASK_SHOW_NIX_STORE_PATH"
        ${coreutils}/bin/echo ""

        # Dependencies
        ${nix}/bin/nix-store --query --tree       "$TASK_SHOW_NIX_STORE_PATH" | ${coreutils}/bin/cat
        ${coreutils}/bin/echo ""
        ${nix}/bin/nix-store --query --references "$TASK_SHOW_NIX_STORE_PATH" | ${coreutils}/bin/cat
        ${nix}/bin/nix-store --query --graph      "$TASK_SHOW_NIX_STORE_PATH" | ${graphviz}/bin/dot -Tpng -o result-dependencies.png 2> /dev/null
      '';
    };

    #
    # Run the project.
    #
    task_run                                                  = taskRunner.mkTask {
      name                                                    = "run";
      dependencies                                            = with pkgs; [
        coreutils           # /bin/echo
        nix                 # /bin/nix
          git               # /bin/git
      ];
      isolate                                                 = false;
      src                                                     = with pkgs; ''
        ${nix}/bin/nix run \
          --experimental-features 'nix-command flakes' \
          --show-trace \
          --option max-jobs 8 \
          --option cores 2 \
          --verbose \
          --option eval-cache false \
          -L \
          "." -- $@
      '';
    };

    #
    # Run generators for examples.
    #
    task_examples                                             = taskRunner.mkTask {
      name                                                    = "examples";
      dependencies                                            = with pkgs; [
        coreutils            # /bin/echo
        asciinema-automation # /bin/asciinema-automation
        asciinema-agg        # /bin/agg
        nix                  # /bin/nix
          git                # /bin/git
        task_run             # /bin/run
      ];
      isolate                                                 = false;
      src                                                     = with pkgs; ''
        # Clear Examples
        ${coreutils}/bin/rm -rf ./examples
        ${coreutils}/bin/mkdir -p ./examples

        # Prime Commands
        ${nix}/bin/nix run github:nate-wilkins/bookit -- --help

        # Record Demo
        ${asciinema-automation}/bin/asciinema-automation .asciinema ./examples/demo.cast
        ${asciinema-agg}/bin/agg ./examples/demo.cast ./examples/demo.gif
      '';
    };

    #
    # Publish project binary.
    #
    task_publish                                             = taskRunner.mkTask {
      name                                                    = "publish";
      dependencies                                            = with pkgs; [
        cargo            # /bin/build        # /bin/test
      ];
      isolate                                                 = false;
      src                                                     = with pkgs; ''
        ${cargo}/bin/cargo login
        ${cargo}/bin/cargo publish
      '';
    };
  in (
    taskRunner.mkTaskRunner {
      dependencies                                       = [
        toolchain
      ] ++ dependencies;
      src                                                = with pkgs; ''
        # Asciinema Isolation
        TMP_HOME="$(${coreutils}/bin/mktemp -d)"
        export HOME=$TMP_HOME

        # Nix Isolation
        ${coreutils}/bin/mkdir -p "$HOME/.config/nix"
        ${coreutils}/bin/echo "experimental-features = nix-command flakes" > "$HOME/.config/nix/nix.conf"

        # Rust
        export RUST_BACKTRACE=1
      '';
      tasks                                              = {
        help                                             = task_help;
        clean                                            = task_clean;
        build                                            = task_build;
        test                                             = task_test;
        docs                                             = task_docs;
        show                                             = task_show;
        run                                              = task_run;
        examples                                         = task_examples;
        publish                                          = task_publish;
      };
    }
  )
)
