#include "main_menu_scene.h"
#include "game_scene.h"
#include "game_state.h"

#include <stdio.h>

void MainMenuScene::update(float /*deltaTime*/, GameState* gs)
{
    if (should_continue)
    {
        gs->switchScene(new GameScene());
    }
    if (should_quit)
    {
        gs->exit();
    }
}
void MainMenuScene::render(Renderer& renderer)
{
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
