#include "game_scene.h"
#include "end_scene.h"

const char* prompts[] = {"CONCENTRATE to make Mario jump!",
                         "Now jump five more times!",
                         "Tense your right arm to make Mario move right!",
                         "Tense your left arm to make Mario move left!",
                         "Now combine jumping and moving to reach the\nflag!"};

int level = 0;
int jumpFiveTimes = 5;

const float jumpForce = 600;
const float moveSpeed = 30;

int rightWalking = 700;
int leftWalking = 700;

void GameScene::jump()
{
    if (isPlayerGrounded)
    {
        playerSpriteVel -= Vec2(0, jumpForce);

        if (level == 1)
        {
            jumpFiveTimes -= 1;
        }

        if (jumpFiveTimes == 0 && level == 1)
        {
            level = 2;
        }

        if (level == 0)
        {
            level += 1;
        }
    }
}

void GameScene::walkLeft()
{
    playerSpriteVel += Vec2(-moveSpeed, 0);
    if (level == 3)
    {
        leftWalking -= 1;
        if (leftWalking <= 0)
        {
            level = 4;
        }
    }
}

void GameScene::walkRight()
{
    playerSpriteVel += Vec2(moveSpeed, 0);
    if (level == 2)
    {
        rightWalking -= 1;
        if (rightWalking <= 0)
        {
            level = 3;
        }
    }
}

void GameScene::menuSelect()
{
    level++;
    constexpr auto numStrings = sizeof(prompts) / sizeof(prompts[0]);
    if (level > 0 && static_cast<std::size_t>(level) >= numStrings)
    {
        level = 0;
    }
}

void GameScene::update(float deltaTime, GameState* gs)
{
    // Handle gravity
    playerSpriteVel += Vec2(0, 9);

    playerSprite.position += playerSpriteVel * deltaTime;

    // Handle downwards
    if (playerSprite.position.y() >= 300.0f)
    {
        playerSprite.position[1] = 300.0f;
        playerSpriteVel[1] = 0.0f;
        isPlayerGrounded = true;
    }
    else
    {
        isPlayerGrounded = false;
    }

    playerSpriteVel[0] = 0.0f;

    if (level == 4)
    {
        printf("Switching...\n");
        gs->switchScene(std::make_unique<EndScene>());
        gs->shouldWrite = true;
    }
}

void GameScene::render(Renderer& renderer)
{
    springBackground.draw(renderer.get_renderer());
    playerSprite.draw(renderer.get_renderer());
    flagSprite.draw(renderer.get_renderer());
    SDL_SetRenderDrawColor(renderer.get_renderer(), 255, 255, 255, 255);
    SDL_RenderFillRect(renderer.get_renderer(), &textbox);
    nextLine(renderer);
}

void GameScene::nextLine(Renderer& renderer)
{
    renderer.draw_text_wrapped(prompts[level], Vec2(5.0f, 405.0f), 690); // width = 700, 5 px padding on each side
}