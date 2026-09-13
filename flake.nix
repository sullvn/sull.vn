{
  description = "sull.vn";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.systems.url = "github:nix-systems/default";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        fontconfigRules = "${pkgs.fontconfig.out}/share/fontconfig/conf.avail";
      in
        {
          devShells.default = pkgs.mkShell {
            packages = [
              pkgs.nodejs_latest
              pkgs.pnpm
              pkgs.biome
              pkgs.actionlint
              pkgs.chromium
            ];
            env = {
              PLAYWRIGHT_BROWSERS_PATH = "${pkgs.playwright-driver.browsers}";
              PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS = "true";
              PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD = "1";

              # Match the existing screenshots without inheriting host or user fonts.
              FONTCONFIG_FILE = pkgs.writeText "sull-vn-fonts.conf" ''
                <?xml version="1.0"?>
                <!DOCTYPE fontconfig SYSTEM "urn:fontconfig:fonts.dtd">
                <fontconfig>
                  <dir>${pkgs.dejavu_fonts.minimal}</dir>
                  <cachedir prefix="xdg">fontconfig</cachedir>
                  <include>${fontconfigRules}/10-hinting-slight.conf</include>
                  <include>${fontconfigRules}/10-yes-antialias.conf</include>
                  <include>${fontconfigRules}/10-sub-pixel-none.conf</include>
                  <include>${fontconfigRules}/11-lcdfilter-default.conf</include>
                  <include>${fontconfigRules}/90-synthetic.conf</include>
                </fontconfig>
              '';
            };
          };
        }

    );
}
