#ifndef WARFIGHTER__GAME_SCENE_H__
#define WARFIGHTER__GAME_SCENE_H__

#include "crc.h"
#include "game_state.h"
#include "scene.h"
#include "sprite.h"
#include "vec2.h"

class GameScene : public Scene
{
private:
    Sprite springBackground;
    SDL_Rect textbox;

    Sprite playerSprite;
    
    Vec2 playerSpriteVel;
    bool isPlayerGrounded = false;

public:
    GameScene()
        : springBackground(Sprite(
              WFID("./data/art/background_scenery_tiles.bmp"),
              Vec2(0, 0),
              Vec2(640, 480),
              Vec2(20, 20),
              Vec2(150, 140)))
        , playerSprite(Sprite(
              WFID("./data/art/super_mario_bros_sprite_sheet.png"),
              Vec2(300, 300),
              Vec2(27, 33),
              Vec2(14, 18),
              Vec2(27, 33)))     
        , textbox(SDL_Rect{0, 400, 700, 100})
        , playerSpriteVel(Vec2(0, 0))
    {}
    virtual void jump() override;
    virtual void walkLeft() override;
    virtual void walkRight() override;
    virtual void menuSelect() override;
    virtual void update(float deltaTime, GameState* gs) override;
    virtual void render(Renderer& renderer) override;
    virtual void nextLine(Renderer& renderer);
};

#endif // WARFIGHTER__GAME_SCENE_H__
