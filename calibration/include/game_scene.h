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

public:
    //GameScene() : springBackground(Sprite(WFID("./data/art/background_scenery_tiles.bmp"), Vec2(0, 0), Vec2(640, 480), Vec2(20, 20), Vec2(150, 140))) {}
    GameScene()
        : springBackground(Sprite(WFID("./data/art/background_scenery_tiles.bmp"), Vec2(0, 0)))
    {}
    virtual void update(float deltaTime, GameState* gs) override;
    virtual void render(Renderer& renderer) override;
};

#endif // WARFIGHTER__GAME_SCENE_H__
