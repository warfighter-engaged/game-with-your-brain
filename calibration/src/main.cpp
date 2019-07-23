#include "SDL.h"
#include <stdio.h>

#include "crc.h"
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

        KeyboardInput ki(&gs);

        Sprite hewwo_sprite(WFID("./data/hewwo.bmp"), Vec2(0.0, 0.0));

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

            // Platforming logic
            // TODO: Extract this into a scene-specific logic section
            // TODO: Get delta time and use that to modify all movement
            gs.y += 1;

            if (gs.y >= SCREEN_HEIGHT)
            {
                gs.y = 0;
            }
            if (gs.x >= SCREEN_WIDTH)
            {
                gs.x = 0;
            }
            if (gs.x < 0)
            {
                gs.x = SCREEN_WIDTH - 1;
            }

            hewwo_sprite.draw(renderer.get_renderer());
            renderer.draw_text("FIGHT ME", Vec2(gs.x, gs.y));

            renderer.present();
        }
    }
};

int main(int argc, char *argv[])
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