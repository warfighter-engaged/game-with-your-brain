#ifndef WARFIGHTER__SCENE_H__
#define WARFIGHTER__SCENE_H__

#include "renderer.h"

class GameState;

class Scene
{
public:
    virtual void update(float deltaTime, GameState* gs) = 0;
    virtual void render(Renderer& renderer) = 0;

    virtual void jump()
    {}
    virtual void walkLeft()
    {}
    virtual void walkRight()
    {}
    virtual void menuSelect()
    {}
    virtual void nextLine()
    {}
    virtual void exit()
    {}
};

#endif // WARFIGHTER__SCENE_H__
