{
  description = "vaultrs";
  inputs = {
    ## Nixpkgs ##
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    ## Std ##
    std.url = "github:divnix/std";
    std.inputs.nixpkgs.follows = "nixpkgs";

    # Rust overlay
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {std, ...} @ inputs:
    std.growOn
    {
      inherit inputs;
      cellsFrom = ./nix;

      cellBlocks = [
        (std.blockTypes.devshells "devshells")
        (std.blockTypes.functions "toolchains")
        (std.blockTypes.nixago "configs")
      ];
    }
    {
      devShells = std.harvest inputs.self ["automation" "devshells"];
    };
}
