#ifndef WARFIGHTER__SPRITE_H__
#define WARFIGHTER__SPRITE_H__

#include <unordered_map>
#include "SDL_image.h"
#include <math.h>

#include "crc.h"
#include "vec2.h"

class Sprite
{
public:
    Sprite(CRC path, Vec2 pos) : spriteId(path), position(pos) {}

    SDL_Texture *get_texture()
    {
        return images[this->spriteId];
    }

    void draw(SDL_Renderer *renderer)
    {
        int width = 0;
        int height = 0;
        SDL_Texture *tex = this->get_texture();
        SDL_QueryTexture(tex, nullptr, nullptr, &width, &height);
        SDL_Rect dst = {(int)position.x(), (int)position.y(), width, height};
        SDL_RenderCopy(renderer, this->get_texture(), nullptr, &dst);
    }

    static bool load_image(const char *path, SDL_Renderer *renderer)
    {
        SDL_Surface *loaded_image = IMG_Load(path);
        if (loaded_image == nullptr)
        {
            printf("Failed to load image %s; SDL_image error: %s\n", path, IMG_GetError());
            return false;
        }
        SDL_Texture *image_texture = SDL_CreateTextureFromSurface(renderer, loaded_image);
        SDL_FreeSurface(loaded_image);
        if (image_texture == nullptr)
        {
            printf("Unable to create texture from %s! SDL error: %s\n", path, SDL_GetError());
            return false;
        }
        images[WFID(path)] = image_texture;
        return true;
    }

    static void unload_images()
    {
        for (auto i : images)
        {
            SDL_DestroyTexture(i.second);
        }
    }

private:
    CRC spriteId;
    Vec2 position;

    static std::unordered_map<CRC, SDL_Texture *> images;
};

#endif // WARFIGHTER__SPRITE_H__