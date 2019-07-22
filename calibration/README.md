# Calibration App/Game

## Installing pre reqs

I've been using WSL and bash, but if you're using Windows, then there are separate instructions over at the link in the references.

``` bash
sudo apt install cmake libsdl2-dev libsdl-image1.2-dev g++
```

## Adding or updating make files

Run inside the folder with the CmakeLists.txt.

``` bash
cmake .
```

## Building the outputs

Run inside the folder where the Makefile was created.

``` bash
make
```

## References

Using SDL 2 with CMake: <https://trenki2.github.io/blog/2017/06/02/using-sdl2-with-cmake/>
