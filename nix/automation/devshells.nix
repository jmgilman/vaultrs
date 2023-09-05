{
  inputs,
  cell,
}: let
  inherit (inputs) nixpkgs std;
  inherit (inputs.cells.lib.toolchains) rustToolchain;
  l = nixpkgs.lib // builtins;

  mkEnv = env: l.mapAttrsToList (name: value: {inherit name value;}) env;

  vaultrs = {...}: {
    name = nixpkgs.lib.mkForce "Vaultrs Devshell";
    env = with nixpkgs;
      mkEnv {
        OPENSSL_NO_VENDOR = 1;
        OPENSSL_DIR = "${l.getDev openssl}";
        OPENSSL_LIB_DIR = "${l.getLib openssl}/lib";
      };
    nixago = [
      cell.configs.lefthook
      cell.configs.prettier
      cell.configs.treefmt
    ];
    packages = with nixpkgs; [
      gcc
      rustToolchain
      pkg-config
    ];
  };
in
  l.mapAttrs (_: std.lib.dev.mkShell) rec {
    default = {...}: {
      imports = [
        vaultrs
      ];
    };
  }
