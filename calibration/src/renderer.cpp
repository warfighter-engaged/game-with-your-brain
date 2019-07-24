#include "renderer.h"
#include "sprite.h"

bool Renderer::init()
{
    // Initialize SDL2.
    // When there's an error, SDL_Init returns -1.
    if (SDL_Init(SDL_INIT_VIDEO) < 0)
    {
        printf("SDL could not initialize! SDL_Error: %s\n", SDL_GetError());
        return false;
    }

    // Initialize the SDL2_ttf library
    if (TTF_Init() < 0)
    {
        printf("Could not initialize SDL_ttf! SDL_Error: %s\n", SDL_GetError());
        return false;
    }

    // And SDL2_image
    int imgFlags = IMG_INIT_PNG;
    if (!(IMG_Init(imgFlags) & imgFlags))
    {
        printf("Could not initialize SDL_image! SDL_Error: %s\n", SDL_GetError());
        return false;
    }

    // Create the window.
    window = SDL_CreateWindow(
        "Calibration", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, SCREEN_WIDTH, SCREEN_HEIGHT, SDL_WINDOW_SHOWN);
    if (window == nullptr)
    {
        printf("Window could not be created! SDL_Error: %s\n", SDL_GetError());
        return false;
    }

#if 0
    if (SDL_SetWindowFullscreen(window, true) < 0)
    {
        printf("Window could not be set to fullscreen! SDL_Error: %s\n", SDL_GetError());
        return false;
    }
#endif

    // Create a renderer
    renderer = SDL_CreateRenderer(window, -1, 0);
    if (renderer == nullptr)
    {
        printf("Could not create renderer. SDL_Error: %s\n", SDL_GetError());
        return false;
    }

    return true;
}

bool Renderer::load_media()
{
    font = TTF_OpenFont("./data/fonts/corbel.ttf", FONT_HEIGHT);
    if (font == nullptr)
    {
        printf("Could not open font. SDL_Error: %s\n", SDL_GetError());
        return false;
    }
    if (!Sprite::load_image("./data/art/background_scenery_tiles.bmp", renderer))
    {
        printf("Could not open image\n");
        return false;
    }  
    if (!Sprite::load_image("./data/art/super_mario_bros_sprite_sheet.png", renderer))
    {
        printf("Could not open image\n");
        return false;
    }
    if (!Sprite::load_image("./data/art/goombas.png", renderer))
    {
        printf("Could not open image\n");
        return false;
    }
    return true;
}

void Renderer::draw_text_wrapped(const char* text, const Vec2& position, uint32_t width)
{
    SDL_Color color = {0, 0, 0};
    SDL_Surface* surface = TTF_RenderText_Blended_Wrapped(font, text, color, (uint32_t)width);
    SDL_Texture* texture = SDL_CreateTextureFromSurface(renderer, surface);

    // We have the text, but we want to display it at the correct (100%) scale.
    // Thus, we need to query how large the source texture is.

    int texW = 0;
    int texH = 0;
    SDL_QueryTexture(texture, nullptr, nullptr, &texW, &texH);
    SDL_Rect dstrect = {(int)position.x(), (int)position.y(), texW, texH};

    // Now we can actually render the texture.
    SDL_RenderCopy(renderer, texture, nullptr, &dstrect);

    SDL_DestroyTexture(texture);
    SDL_FreeSurface(surface);
}

void Renderer::draw_text(const char* text, const Vec2& position, uint8_t r, uint8_t g, uint8_t b)
{
    SDL_Color color = {r, g, b};
    SDL_Surface* surface = TTF_RenderText_Blended(font, text, color);
    SDL_Texture* texture = SDL_CreateTextureFromSurface(renderer, surface);

    // We have the text, but we want to display it at the correct (100%) scale.
    // Thus, we need to query how large the source texture is.

    int texW = 0;
    int texH = 0;
    SDL_QueryTexture(texture, nullptr, nullptr, &texW, &texH);
    SDL_Rect dstrect = {(int)position.x(), (int)position.y(), texW, texH};

    // Now we can actually render the texture.
    SDL_RenderCopy(renderer, texture, nullptr, &dstrect);

    SDL_DestroyTexture(texture);
    SDL_FreeSurface(surface);
}

Renderer::~Renderer()
{
    Sprite::unload_images();

    if (font != nullptr)
    {
        TTF_CloseFont(font);
    }

    if (renderer != nullptr)
    {
        SDL_DestroyRenderer(renderer);
    }

    if (window != nullptr)
    {
        SDL_DestroyWindow(window);
    }

    TTF_Quit();
    SDL_Quit();
}