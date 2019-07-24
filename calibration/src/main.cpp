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
#include "keyboard_input.h"
#include "serial_input.h"

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
        SerialInput si(&gs);

        while (!gs.shouldExit)
        {
            ki.retrigger();
            si.retrigger();
            while (SDL_PollEvent(&e) != 0)
            {
                if (e.type == SDL_QUIT)
                {
                    gs.exit();
                }
                ki.interpret(e);
            }
            si.poll_data();

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

            if (gs.shouldWrite)
            {
                si.write_thresholds();
                gs.shouldWrite = false;
            }

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
