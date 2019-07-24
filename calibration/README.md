# Calibration App/Game

## Installing pre reqs

I've been using WSL and bash, but if you're using Windows, then there are separate instructions over at the link in the references.

``` bash
sudo apt install cmake libsdl2-dev libsdl-image1.2-dev g++
```

## Adding or updating make files

``` bash
mkdir build
cd build
cmake ..
```

## Building the outputs

Run inside the folder where the Makefile was created.

``` bash
make
```

## References

Using SDL 2 with CMake: <https://trenki2.github.io/blog/2017/06/02/using-sdl2-with-cmake/>

## Windows Install

1. Download the Windows development library from <https://www.libsdl.org/release/SDL2-devel-2.0.9-VC.zip> and unzip to e.g. `C:\SDL2-2.0.9`
2. Download the SDL2_image development library from <https://www.libsdl.org/projects/SDL_image/release/SDL2_image-devel-2.0.5-VC.zip> and unzip to e.g. `C:\SDL2_image-2.0.5`
3. Download the SDL_ttf development library from <https://www.libsdl.org/projects/SDL_ttf/release/SDL2_ttf-devel-2.0.15-VC.zip> and unzip to e.g. `C:\SDL2_ttf-2.0.15`
4. Download the SDL2_mixer development library from <https://www.libsdl.org/projects/SDL_mixer/release/SDL2_mixer-2.0.4.zip> and unzip to e.g. `C:\SDL2_mixer-2.0.4`
5. Set the `SDL2DIR` environment variable to `C:\SDL2-2.0.9`
6. Set the `SDL2IMAGEDIR` environment variable to `C:\SDL2_image-2.0.5`
7. Set the `SDL2TTFDIR` environment variable to `C:\SDL2_ttf-2.0.15`
8. Set the `SDL2MIXERDIR` environment variable to `C:\SDL2_mixer-2.0.4`
9. Run vcvars64.bat - assuming you have VS 2019 community, this should be `"C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64"`
10. Inside our project directory under calibration, run `mkdir build`
11. `cd build`
12. `cmake ..`
13. `devenv /build Debug calibration.sln` - you can also open the solution in visual studio
14. `.\Debug\calibration.exe`
