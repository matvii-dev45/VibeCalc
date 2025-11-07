{
  description = "Graphic dependencies";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        # Graphics - X11
        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
        xorg.libXinerama
        xorg.libxcb
        xorg.libXxf86vm
        
        # Graphics - Wayland
        libxkbcommon
        wayland
        
        # OpenGL/Vulkan
        mesa
        libGL
        libGLU
        
        # Vulkan
        vulkan-loader
        vulkan-validation-layers
        vulkan-tools
        
        # EGL (for eframe)
        libglvnd
        
        # Audio
        alsa-lib
        pulseaudio
        
        # Build tools
        pkg-config
        cmake
      ];

      shellHook = ''
        export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
          pkgs.xorg.libX11
          pkgs.xorg.libXcursor
          pkgs.xorg.libXi
          pkgs.xorg.libXrandr
          pkgs.xorg.libXinerama
          pkgs.xorg.libxcb
          pkgs.xorg.libXxf86vm
          pkgs.libxkbcommon
          pkgs.wayland
          pkgs.mesa
          pkgs.libGL
          pkgs.libGLU
          pkgs.libglvnd
          pkgs.vulkan-loader
          pkgs.vulkan-validation-layers
          pkgs.alsa-lib
          pkgs.pulseaudio
        ]}:$LD_LIBRARY_PATH

        # Vulkan
        export VK_ICD_FILENAMES="${pkgs.mesa.drivers}/share/vulkan/icd.d/radeon_icd.x86_64.json:${pkgs.mesa.drivers}/share/vulkan/icd.d/intel_icd.x86_64.json:${pkgs.mesa.drivers}/share/vulkan/icd.d/lvp_icd.x86_64.json"
        export VK_LAYER_PATH="${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d"
        
        # OpenGL
        export LIBGL_DRIVERS_PATH="${pkgs.mesa.drivers}/lib/dri"
        export __GLX_VENDOR_LIBRARY_NAME="mesa"
        
        # EGL (критично для eframe/egui!)
        export EGL_PLATFORM=x11
        export WINIT_UNIX_BACKEND=x11

        echo "✅ Graphics env ready"
        echo "   OpenGL: ${pkgs.mesa.drivers}/lib/dri"
        echo "   Backend: X11"
      '';
    };
  };
}