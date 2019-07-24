#include "game_scene.h"

const char* prompts[] = {"CONCENTRATE to make Mario jump!",
                         "Now jump five more times!",
                         "Tense your right arm to make Mario move right!",
                         "Tense your left arm to make Mario move left!",
                         "Now combine jumping and moving to reach the\nflag!"};

int level = 0;

void GameScene::jump()
{
    playerSpriteVel += Vec2(0, 20);
}

void GameScene::walkLeft()
{
    playerSpriteVel += Vec2(-10, 0);
}

void GameScene::walkRight()
{
    playerSpriteVel += Vec2(10, 0);
}

void GameScene::menuSelect()
{
    level++;
    size_t numStrings = sizeof(prompts) / sizeof(prompts[0]);
    if (level >= numStrings)
    {
        level = 0;
    }
}

void GameScene::update(float deltaTime, GameState* /*gs*/)
{
    playerSprite.position += playerSpriteVel * deltaTime;

    // Handle gravity
    playerSprite.position += Vec2(0, 9);

    // Handle downwards
    if (playerSprite.position.y() > 300.0f)
    {
        playerSprite.position[1] = 300.0f;
    }

    playerSpriteVel = Vec2(0, 0);
}
void GameScene::render(Renderer& renderer)
{
    springBackground.draw(renderer.get_renderer());
    playerSprite.draw(renderer.get_renderer());
    SDL_SetRenderDrawColor(renderer.get_renderer(), 255, 255, 255, 255);
    SDL_RenderFillRect(renderer.get_renderer(), &textbox);
    nextLine(renderer);
}

void GameScene::nextLine(Renderer& renderer)
{
    renderer.draw_text_wrapped(prompts[level], Vec2(5.0f, 405.0f), 690); // width = 700, 5 px padding on each side
}