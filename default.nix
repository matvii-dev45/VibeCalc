{ lib
, rustPlatform
, fetchFromGitHub
, pkg-config
, cmake
, xorg
, libxkbcommon
, wayland
, mesa
, libGL
, libGLU
, libglvnd
, vulkan-loader
, vulkan-validation-layers
, alsa-lib
, pulseaudio
, makeWrapper
}:

rustPlatform.buildRustPackage rec {
  pname = "vibe-calc";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "matvii-dev45";
    repo = "VibeCalc";
    rev = "v${version}";
    sha256 = lib.fakeSha256;
  };

  cargoHash = lib.fakeHash;

  nativeBuildInputs = [
    pkg-config
    cmake
    makeWrapper
  ];

  buildInputs = [
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    xorg.libXinerama
    xorg.libxcb
    xorg.libXxf86vm
    libxkbcommon
    wayland
    mesa
    libGL
    libGLU
    libglvnd
    vulkan-loader
    vulkan-validation-layers
    alsa-lib
    pulseaudio
  ];

  postInstall = ''
    wrapProgram $out/bin/vibe-calc \
      --prefix LD_LIBRARY_PATH : "${lib.makeLibraryPath [
        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
        xorg.libXinerama
        xorg.libxcb
        xorg.libXxf86vm
        libxkbcommon
        wayland
        mesa
        libGL
        libGLU
        libglvnd
        vulkan-loader
        vulkan-validation-layers
        alsa-lib
        pulseaudio
      ]}" \
      --set VK_ICD_FILENAMES "${mesa.drivers}/share/vulkan/icd.d/radeon_icd.x86_64.json:${mesa.drivers}/share/vulkan/icd.d/intel_icd.x86_64.json:${mesa.drivers}/share/vulkan/icd.d/lvp_icd.x86_64.json" \
      --set VK_LAYER_PATH "${vulkan-validation-layers}/share/vulkan/explicit_layer.d" \
      --set LIBGL_DRIVERS_PATH "${mesa.drivers}/lib/dri" \
      --set __GLX_VENDOR_LIBRARY_NAME "mesa" \
      --set EGL_PLATFORM "x11" \
      --set WINIT_UNIX_BACKEND "x11"
  '';

  meta = with lib; {
    description = "A calculator with customizable GUI built with egui";
    homepage = "https://github.com/YOUR_GITHUB_USERNAME/vibe-calc";
    license = licenses.mit;
    maintainers = [ ];
    mainProgram = "vibe-calc";
    platforms = platforms.linux;
  };
}