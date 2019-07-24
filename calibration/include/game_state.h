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
    void nextLine();
    void menuSelect();
    void exit();

    void switchScene(std::unique_ptr<Scene>&& newScene)
    {
        loadScene = std::move(newScene);
    }

    bool shouldExit = false;

    std::unique_ptr<Scene> currentScene = nullptr;
    std::unique_ptr<Scene> loadScene = nullptr;
};

#endif // WARFIGHTER__GAME_STATE_H__
