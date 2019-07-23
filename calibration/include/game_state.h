#ifndef WARFIGHTER__GAME_STATE_H__
#define WARFIGHTER__GAME_STATE_H__

#include "scene.h"

class GameState
{
public:
    void jump();
    void walkLeft();
    void walkRight();
    void menuSelect();
    void exit();

    void switchScene(Scene *newScene)
    {
        if (loadScene != nullptr)
        {
            delete loadScene;
        }
        loadScene = newScene;
    }

    bool shouldExit = false;
    int x = 0;
    int y = 0;

    Scene *currentScene = nullptr;
    Scene *loadScene = nullptr;
};

#endif // WARFIGHTER__GAME_STATE_H__
