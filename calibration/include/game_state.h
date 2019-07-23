#ifndef WARFIGHTER__GAME_STATE_H__
#define WARFIGHTER__GAME_STATE_H__

#include "scene.h"
#include <memory>

class GameState
{
public:
    void jump();
    void walkLeft();
    void walkRight();
    void menuSelect();
    void exit();

    void switchScene(std::unique_ptr<Scene>&& newScene)
    {
        loadScene = std::move(newScene);
    }

    bool shouldExit = false;
    int x = 0;
    int y = 0;

    std::unique_ptr<Scene> currentScene = nullptr;
    std::unique_ptr<Scene> loadScene = nullptr;
};

#endif // WARFIGHTER__GAME_STATE_H__
