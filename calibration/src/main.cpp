#include "SDL.h"
#include <stdio.h>

#include "game_state.h"
#include "input.h"

#define FONT_HEIGHT 32

#include "SDL_ttf.h"

const int SCREEN_WIDTH = 640;
const int SCREEN_HEIGHT = 480;

struct KeyboardInput {
  Command *w;
  Command *a;
  Command *d;
  Command *enter;
  Command *esc;

  KeyboardInput(GameState *gs)
      : w(new JumpCommand(gs)), a(new WalkLeftCommand(gs)),
        d(new WalkRightCommand(gs)), enter(new MenuSelectCommand(gs)), esc(new ExitCommand(gs)) {}

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
			break;
		  case SDLK_ESCAPE:
			esc->execute();
			break;
      }
    }
  }
};

struct Game {
  SDL_Window *window;
  SDL_Surface *screenSurface;
  SDL_Surface *helloWorld;
  SDL_Renderer *renderer;
  TTF_Font *font;
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

	SDL_SetWindowFullscreen(window, true);

    SDL_Init(SDL_INIT_VIDEO);

    // Initialize the SDL_ttf library
    TTF_Init();

    // Get the window surface
    screenSurface = SDL_GetWindowSurface(window);

    // Create a renderer
    renderer = SDL_CreateRenderer(window, -1, 0);

    return true;
  }

  bool loadMedia() {
    // Smiley face image
    helloWorld = SDL_LoadBMP("./data/hewwo.bmp");
    if (helloWorld == NULL) {
      printf("Unable to load image %s! SDL Error: %s\n", "hewwo",
             SDL_GetError());
      return false;
    }

    font = TTF_OpenFont("./data/fonts/corbel.ttf", FONT_HEIGHT);

    return true;
  }

  void game_loop() {
    SDL_Event e;
    GameState gs;

    KeyboardInput ki(&gs);

	int x = 0, y = 0;

    while (!quit && !gs.shouldExit) {

      while (SDL_PollEvent(&e) != 0) {
        if (e.type == SDL_QUIT) {
          quit = true;
        }
        ki.interpret(e);
      }

      x += 1;
      y += 1;

      if (x > SCREEN_WIDTH) {
        x = 0;
      }
      if (y > SCREEN_HEIGHT) {
        y = 0;
      }

      {
        SDL_Texture *texture =
            SDL_CreateTextureFromSurface(renderer, helloWorld);
        SDL_RenderCopy(renderer, texture, NULL, NULL);

        SDL_DestroyTexture(texture);
      }

      {
        SDL_Color color = {0, 0, 0};
        SDL_Surface *surface =
            TTF_RenderText_Blended(font, "Bonjour, mon ami!", color);
        SDL_Texture *texture = SDL_CreateTextureFromSurface(renderer, surface);

        int texW = 0;
        int texH = 0;
        SDL_QueryTexture(texture, NULL, NULL, &texW, &texH);
        SDL_Rect dstrect = {x, y, texW, texH};

        SDL_RenderCopy(renderer, texture, NULL, &dstrect);
        SDL_DestroyTexture(texture);
        SDL_FreeSurface(surface);
      }
      SDL_RenderPresent(renderer);
    }
  }

  void close() {
    // Destroy font
    TTF_CloseFont(font);

    // Deallocate surface
    SDL_FreeSurface(helloWorld);

    // Destroy the renderer
    SDL_DestroyRenderer(renderer);

    // Destroy the window
    SDL_DestroyWindow(window);

    TTF_Quit();
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