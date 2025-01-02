{ pkgs ? (import <nixpkgs> {}) }:
with pkgs;
mkShell {
  buildInputs = [
    rustup
    cmake

    # Libraries
    libGL
    xorg.libX11
    xorg.libXi
    libxkbcommon
  ];

  # Env variabels
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
      with pkgs;
      pkgs.lib.makeLibraryPath [ libGL xorg.libX11 xorg.libXi libxkbcommon ]
    }"
  '';
}
