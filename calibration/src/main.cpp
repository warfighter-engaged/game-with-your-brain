#include "SDL.h"
#include <stdio.h>

#include "game_state.h"
#include "input.h"

const int SCREEN_WIDTH = 640;
const int SCREEN_HEIGHT = 480;

struct KeyboardInput {
  Command *w;
  Command *a;
  Command *d;
  Command *enter;

  KeyboardInput(GameState *gs)
      : w(new JumpCommand(gs)), a(new WalkLeftCommand(gs)),
        d(new WalkRightCommand(gs)), enter(new MenuSelectCommand(gs)) {}

  void interpret(SDL_Event const &e) {
    if (e.type == SDL_KEYDOWN) {
      switch (e.key.keysym.sym) {
      case SDLK_w:
        w->execute();
        break;
      case SDLK_a:
        a->execute();
        break;
      case SDLK_d:
        d->execute();
        break;
      case SDLK_RETURN:
        enter->execute();
      }
    }
  }
};

struct Game {
  SDL_Window *window;
  SDL_Surface *screenSurface;
  SDL_Surface *helloWorld;
  bool quit;

  Game() : window(NULL), screenSurface(NULL), helloWorld(NULL), quit(false) {}

  bool init() {
    // Initialize SDL2.
    // When there's an error, SDL_Init returns -1.
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
      printf("SDL could not initialize! SDL_Error: %s\n", SDL_GetError());
      return false;
    }

    // Create the window.
    window = SDL_CreateWindow("Calibration", SDL_WINDOWPOS_UNDEFINED,
                              SDL_WINDOWPOS_UNDEFINED, SCREEN_WIDTH,
                              SCREEN_HEIGHT, SDL_WINDOW_SHOWN);
    if (window == NULL) {
      printf("Window could not be created! SDL_Error: %s\n", SDL_GetError());
      return false;
    }

    SDL_Init(SDL_INIT_VIDEO);

    // Get the window surface
    screenSurface = SDL_GetWindowSurface(window);
    return true;
  }

  bool loadMedia() {
    helloWorld = SDL_LoadBMP("./data/hewwo.bmp");
    if (helloWorld == NULL) {
      printf("Unable to load image %s! SDL Error: %s\n", "hewwo",
             SDL_GetError());
      return false;
    }
    return true;
  }

  void game_loop() {
    SDL_Event e;
    GameState gs;

    KeyboardInput ki(&gs);

    while (!quit) {
      while (SDL_PollEvent(&e) != 0) {
        if (e.type == SDL_QUIT) {
          quit = true;
        }
        ki.interpret(e);
      }
      // Fill the surface white
      SDL_FillRect(screenSurface, NULL,
                   SDL_MapRGB(screenSurface->format, 0xFF, 0xFF, 0xFF));
      // Apply the image
      SDL_BlitSurface(helloWorld, NULL, screenSurface, NULL);
      // Update the surface
      SDL_UpdateWindowSurface(window);
    }
  }

  void close() {
    // Deallocate surface
    SDL_FreeSurface(helloWorld);
    SDL_DestroyWindow(window);
    SDL_Quit();
  }
};

int main(int argc, char *argv[]) {
  bool result;
  Game game;
  result = game.init();
  if (!result) {
    printf("Failed to initialize!\n");
    return 1;
  }

  result = game.loadMedia();
  if (!result) {
    printf("Failed to load media!\n");
    return 1;
  }

  game.game_loop();

  game.close();

  return 0;
}