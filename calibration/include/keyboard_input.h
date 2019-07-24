#ifndef WARFIGHTER__KEYBOARD_INPUT_H__
#define WARFIGHTER__KEYBOARD_INPUT_H__

#include <memory>
#include "input.h"
#include "SDL.h"

// TODO: Make events that fire on hold (per-frame if the key is down instead of just the frame when the key is pressed)
// not 100% sure the best way to do this
struct KeyboardInput
{
    bool w_down = false;
    bool a_down = false;
    bool d_down = false;
    bool t_down = false;
    bool enter_down = false;
    bool esc_down = false;

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
                w_down = true;
                w->execute();
                break;
            case SDLK_a:
                a_down = true;
                a->execute();
                break;
            case SDLK_d:
                d_down = true;
                d->execute();
                break;
            case SDLK_t:
                t_down = true;
                t->execute();
                break;
            case SDLK_RETURN:
                enter_down = true;
                enter->execute();
                break;
            case SDLK_ESCAPE:
                esc_down = true;
                esc->execute();
                break;
            }
        }
        else if (e.type == SDL_KEYUP)
        {
            switch (e.key.keysym.sym)
            {
            case SDLK_w:
                w_down = false;
                break;
            case SDLK_a:
                a_down = false;
                break;
            case SDLK_d:
                d_down = false;
                break;
            case SDLK_t:
                t_down = false;
                break;
            case SDLK_RETURN:
                enter_down = false;
                break;
            case SDLK_ESCAPE:
                esc_down = false;
                break;
            }
        }
    }

    void retrigger()
    {
        if (w->retrigger && w_down)
        {
            w->execute();
        }

        if (a->retrigger && a_down)
        {
            a->execute();
        }

        if (d->retrigger && d_down)
        {
            d->execute();
        }

        if (t->retrigger && t_down)
        {
            t->execute();
        }

        if (enter->retrigger && enter_down)
        {
            enter->execute();
        }

        if (esc->retrigger && esc_down)
        {
            enter->execute();
        }
    }
};

#endif // WARFIGHTER__KEYBOARD_INPUT_H__