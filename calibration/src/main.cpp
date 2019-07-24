#include "SDL.h"
#include <stdio.h>
#include <stdint.h>
#include <sstream>

#include "crc.h"
#include "main_menu_scene.h"
#include "game_state.h"
#include "input.h"
#include "renderer.h"
#include "sprite.h"

// TODO: Make events that fire on hold (per-frame if the key is down instead of just the frame when the key is pressed)
// not 100% sure the best way to do this
struct KeyboardInput
{
    std::unique_ptr<Command> w;
    std::unique_ptr<Command> a;
    std::unique_ptr<Command> d;
    std::unique_ptr<Command> t;
    std::unique_ptr<Command> enter;
    std::unique_ptr<Command> esc;

    KeyboardInput(GameState* gs)
        : w(std::make_unique<JumpCommand>(gs))
        , a(std::make_unique<WalkLeftCommand>(gs))
        , d(std::make_unique<WalkRightCommand>(gs))
        , enter(std::make_unique<MenuSelectCommand>(gs))
        , esc(std::make_unique<ExitCommand>(gs))
        , t(std::make_unique<NextLineCommand>(gs))
    {}

    void interpret(SDL_Event const& e)
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
            case SDLK_t:
                t->execute();
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

    Game()
    {}

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
        uint64_t now = SDL_GetPerformanceCounter();
        uint64_t last = 0;
        float deltaTime = 0;

        SDL_Event e;
        GameState gs;
        gs.currentScene = std::make_unique<MainMenuScene>();

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

            last = now;
            now = SDL_GetPerformanceCounter();
            deltaTime = (float)((now - last) / (float)SDL_GetPerformanceFrequency());

            // Clear the renderer to black
            SDL_SetRenderDrawColor(renderer.get_renderer(), 0, 0, 0, 255);
            renderer.clear();

            // Update and draw the current scene
            // TODO: Get the frame delay as delta time
            gs.currentScene->update(deltaTime, &gs);
            gs.currentScene->render(renderer);

            std::ostringstream oss;
            oss << "FPS: " << 1.0 / deltaTime;

            renderer.draw_text(oss.str().c_str(), Vec2(0.0, 0.0), 255, 0, 0);

            if (gs.loadScene)
            {
                gs.currentScene = std::move(gs.loadScene);
            }
            renderer.present();
        }
    }
};

int main(int /*argc*/, char* /*argv*/ [])
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
