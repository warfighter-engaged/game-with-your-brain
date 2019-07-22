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

> TODO: Still breaking "unable to find SDL.h"

1. Download the Windows development library from <https://www.libsdl.org/release/SDL2-devel-2.0.9-VC.zip> and unzip to e.g. `C:\SDL2-2.0.9`
2. Download the SDL2_image development library from <https://www.libsdl.org/projects/SDL_image/release/SDL2_image-devel-2.0.5-VC.zip> and unzip to e.g. `C:\SDL2_image-2.0.5`
3. Set the `SDL2DIR` environment variable to `C:\SDL2-2.0.9`
4. Run vcvars64.bat - assuming you have VS 2019 community, this should be `"C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64"`
5. `mkdir build`
6. `cd build`
7. `cmake ..`
8. `devenv /build Debug SDL2Test.sln` - you can also open the solution in visual studio
9. `.\Debug\SDL2Test.exe`
