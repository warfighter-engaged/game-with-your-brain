#ifndef WARFIGHTER__RENDERER_H__
#define WARFIGHTER__RENDERER_H__

#include "SDL.h"
#include "SDL_ttf.h"
#include "SDL_image.h"
#include <stdio.h>
#include <stdint.h>

#include "vec2.h"

const int SCREEN_WIDTH = 640;
const int SCREEN_HEIGHT = 480;
const int FONT_HEIGHT = 32;

// Handles rendering sprites and text to the screen.
// Destroys resources when it's done (as opposed to SDL2, which is a C library
// and thus relies upon manual destruction of resources).

class Renderer
{
private:
    SDL_Window* window;
    SDL_Renderer* renderer;
    TTF_Font* font = nullptr;

public:
    Renderer()
        : window(nullptr)
        , renderer(nullptr)
    {}
    bool init();
    bool load_media();

    void draw_text(const char* text, const Vec2& position)
    {
        draw_text(text, position, 0, 0, 0);
    }
    void draw_text(const char* text, const Vec2& position, uint8_t r, uint8_t g, uint8_t b);
    void clear()
    {
        SDL_RenderClear(renderer);
    }
    void present()
    {
        SDL_RenderPresent(renderer);
    }
    SDL_Renderer* get_renderer() const
    {
        return renderer;
    }

    ~Renderer();
};

#endif // WARFIGHTER__RENDERER_H__
