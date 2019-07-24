#include "main_menu_scene.h"
#include "game_scene.h"
#include "game_state.h"

#include <stdio.h>
#include <memory>

void MainMenuScene::update(float /*deltaTime*/, GameState* gs)
{
    if (should_continue)
    {
        printf("Switching...\n");
        gs->switchScene(std::make_unique<GameScene>());

        should_continue = false;
    }
    if (should_quit)
    {
        gs->exit();
        should_quit = false;
    }
}
void MainMenuScene::render(Renderer& renderer)
{
    summerBackground.draw(renderer.get_renderer());
    menu.draw(renderer);
}
void MainMenuScene::walkLeft()
{
    menu.prev();
}
void MainMenuScene::walkRight()
{
    menu.next();
}
void MainMenuScene::menuSelect()
{
    if (menu.selection() == 0)
    {
        printf("Number 1!\n");
        should_continue = true;
    }
    else
    {
        should_quit = true;
        printf("Number 2!\n");
    }
}
