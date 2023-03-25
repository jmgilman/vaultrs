{
  inputs,
  cell,
}: let
  inherit (inputs) nixpkgs std;
  l = nixpkgs.lib // builtins;
  inherit (inputs.cells.lib.toolchains) rustToolchain;
in {
  # TODO: Potentially enable this
  conform = std.std.nixago.conform {
    data = {
      commit = {
        header = {length = 89;};
        conventional = {
          types = [
            "build"
            "chore"
            "ci"
            "docs"
            "feat"
            "fix"
            "perf"
            "refactor"
            "style"
            "test"
          ];
          scopes = [
            "devshell"
            "jormungandr"
          ];
        };
      };
    };
  };
  lefthook = std.std.nixago.lefthook {
    data = {
      # TODO: Potentially enable this
      # commit-msg = {
      #   commands = {
      #     conform = {
      #       run = "${nixpkgs.conform}/bin/conform enforce --commit-msg-file {1}";
      #     };
      #   };
      # };
      pre-commit = {
        commands = {
          treefmt = {
            run = "${nixpkgs.treefmt}/bin/treefmt --fail-on-change {staged_files}";
          };
          rustfmt = {
            run = "${rustToolchain}/bin/cargo fmt";
          };
        };
      };
    };
  };
  prettier =
    std.lib.dev.mkNixago
    {
      data = {
        printWidth = 80;
        proseWrap = "always";
      };
      output = ".prettierrc";
      format = "json";
      packages = with nixpkgs; [nodePackages.prettier];
    };
  treefmt =
    std.std.nixago.treefmt
    {
      data = {
        formatter = {
          nix = {
            command = "alejandra";
            includes = ["*.nix"];
          };
          prettier = {
            command = "prettier";
            options = ["--write"];
            includes = [
              "*.md"
            ];
          };
        };
      };
      packages = with nixpkgs; [alejandra];
    };
}
