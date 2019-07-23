#ifndef WARFIGHTER__MAIN_MENU_SCENE_H__
#define WARFIGHTER__MAIN_MENU_SCENE_H__

#include <vector>
#include <string>

#include "menu.h"
#include "scene.h"
#include "sprite.h"
#include "vec2.h"

class MainMenuScene : public Scene
{
private:
    Menu menu;
    bool should_continue;
    bool should_quit;

public:
    MainMenuScene() : menu(Menu(std::vector<std::string>{"Hello, world", "Goodbye, world"})), should_continue(false), should_quit(false) {}

    virtual void update(float deltaTime, GameState *gs) override;
    virtual void render(Renderer &renderer) override;
    virtual void walkLeft() override;
    virtual void walkRight() override;
    virtual void menuSelect() override;
};

#endif // WARFIGHTER__MAIN_MENU_SCENE_H__
