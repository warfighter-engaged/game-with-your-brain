#include "game_state.h"
#include <stdio.h>

void GameState::jump()
{
    printf("Jumping!\n");
    y += 8;
}

void GameState::walkLeft()
{
    printf("Walking left\n");
    x -= 1;
}

void GameState::walkRight()
{
    printf("Walking right\n");
    x += 1;
}

void GameState::menuSelect() { printf("Menu select\n"); }

void GameState::exit()
{
    printf("Exit\n");
    shouldExit = true;
}