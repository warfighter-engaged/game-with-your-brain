#include "game_state.h"
#include <stdio.h>

void GameState::jump()
{
    currentScene->jump();
}

void GameState::walkLeft()
{
    currentScene->walkLeft();
}

void GameState::walkRight()
{
    currentScene->walkRight();
}

void GameState::menuSelect()
{
    currentScene->menuSelect();
}

void GameState::nextLine()
{
    currentScene->nextLine();
}

void GameState::exit()
{
    printf("Exit\n");
    shouldExit = true;
}