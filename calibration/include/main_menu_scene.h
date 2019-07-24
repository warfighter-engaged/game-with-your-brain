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
    Sprite summerBackground;

public:
    MainMenuScene() : menu(Menu(std::vector<std::string>{"Start", "Exit"})), should_continue(false), should_quit(false), 
		summerBackground(Sprite(WFID("./data/art/background_scenery_tiles.bmp"), Vec2(0, 0), Vec2(640, 480), Vec2(20, 240), Vec2(150, 140)))
    {}

    virtual void update(float deltaTime, GameState *gs) override;
    virtual void render(Renderer &renderer) override;
    virtual void walkLeft() override;
    virtual void walkRight() override;
    virtual void menuSelect() override;
};

#endif // WARFIGHTER__MAIN_MENU_SCENE_H__
