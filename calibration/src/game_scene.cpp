#include "game_scene.h"

void GameScene::update(float deltaTime)
{
}
void GameScene::render(Renderer &renderer)
{
    hewwoSprite.draw(renderer.get_renderer());
    renderer.draw_text("HEWWO?????", Vec2(0.0f, 0.0f));
}