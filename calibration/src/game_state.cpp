#include "game_state.h"
#include <stdio.h>

void GameState::jump() { printf("Jumping!\n"); }

void GameState::walkLeft() { printf("Walking left\n"); }

void GameState::walkRight() { printf("Walking right\n"); }

void GameState::menuSelect() { printf("Menu select\n"); }

void GameState::exit() { printf("Exit\n"); shouldExit = true; }