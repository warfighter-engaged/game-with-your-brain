#ifndef WARFIGHTER__END_SCENE_H__
#define WARFIGHTER__END_SCENE_H__

#include "crc.h"
#include "game_state.h"
#include "scene.h"
#include "sprite.h"
#include "vec2.h"

class EndScene : public Scene
{
private:
    Sprite winterBackground;
    SDL_Rect textbox;
public:
    EndScene()
        : winterBackground(Sprite(
              WFID("./data/art/background_scenery_tiles.bmp"),
              Vec2(0, 0),
              Vec2(640, 480),
              Vec2(240, 20),
              Vec2(150, 140)))
        , textbox(SDL_Rect{0, 400, 700, 100})
    {}
    virtual void update(float deltaTime, GameState* gs) override;
    virtual void render(Renderer& renderer) override;
    virtual void nextLine(Renderer& renderer);
};

#endif // WARFIGHTER__END_SCENE_H__