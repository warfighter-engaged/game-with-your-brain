#include "SDL.h"
#include <stdio.h>

#include "crc.h"
#include "game_scene.h"
#include "game_state.h"
#include "input.h"
#include "renderer.h"
#include "sprite.h"

// TODO: Make events that fire on hold (per-frame if the key is down instead of just the frame when the key is pressed)
// not 100% sure the best way to do this
struct KeyboardInput
{
    Command *w;
    Command *a;
    Command *d;
    Command *enter;
    Command *esc;

    KeyboardInput(GameState *gs)
        : w(new JumpCommand(gs)), a(new WalkLeftCommand(gs)),
          d(new WalkRightCommand(gs)), enter(new MenuSelectCommand(gs)),
          esc(new ExitCommand(gs)) {}

    void interpret(SDL_Event const &e)
    {
        if (e.type == SDL_KEYDOWN)
        {
            switch (e.key.keysym.sym)
            {
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

struct Game
{
    Renderer renderer;

    Game() {}

    bool init()
    {
        if (!renderer.init())
        {
            printf("Renderer initialization failed\n");
            return false;
        }

        if (!renderer.load_media())
        {
            printf("Media resolution failed\n");
            return false;
        }

        return true;
    }

    void game_loop()
    {
        SDL_Event e;
        GameState gs;
        gs.currentScene = new GameScene();

        KeyboardInput ki(&gs);

        while (!gs.shouldExit)
        {

            while (SDL_PollEvent(&e) != 0)
            {
                if (e.type == SDL_QUIT)
                {
                    gs.exit();
                }
                ki.interpret(e);
            }

            // Clear the renderer
            renderer.clear();

            // Update and draw the current scene
            // TODO: Get the frame delay as delta time
            gs.currentScene->update(0.0f);
            gs.currentScene->render(renderer);

            if (gs.loadScene != nullptr)
            {
                delete gs.currentScene;
                gs.currentScene = gs.loadScene;
                gs.loadScene = nullptr;
            }

            renderer.present();
        }
    }
};

int main(int /*argc*/, char * /*argv*/[])
{
    bool result;
    Game game;
    result = game.init();
    if (!result)
    {
        printf("Failed to initialize!\n");
        return 1;
    }

    game.game_loop();

    return 0;
}