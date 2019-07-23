#ifndef WARFIGHTER__GAME_SCENE_H__
#define WARFIGHTER__GAME_SCENE_H__

#include "crc.h"
#include "scene.h"
#include "sprite.h"
#include "vec2.h"

class GameScene : public Scene
{
private:
    Sprite hewwoSprite;

public:
    GameScene() : hewwoSprite(Sprite(WFID("./data/hewwo.bmp"), Vec2(0.0, 0.0))) {}

    virtual void update(float deltaTime) override;
    virtual void render(Renderer &renderer) override;
};

#endif // WARFIGHTER__GAME_SCENE_H__
