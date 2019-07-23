#ifndef WARFIGHTER__SCENE_H__
#define WARFIGHTER__SCENE_H__

#include "renderer.h"

class Scene
{
public:
    virtual void update(float deltaTime) = 0;
    virtual void render(Renderer &renderer) = 0;
};

#endif // WARFIGHTER__SCENE_H__
