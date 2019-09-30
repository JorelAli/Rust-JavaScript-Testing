{ pkgs ? import <nixpkgs> {} }:

with pkgs;

stdenv.mkDerivation {
  name = "webgtk";

  buildInputs = [
    glib
    gsettings-desktop-schemas
    gtk3
  ];

  nativeBuildInputs = [ wrapGAppsHook ];

  shellHook = ''
    export XDG_DATA_DIRS="$GSETTINGS_SCHEMAS_PATH";
  '';
}
