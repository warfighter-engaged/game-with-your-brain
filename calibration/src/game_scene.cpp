#include "game_scene.h"

void GameScene::update(float /*deltaTime*/, GameState* /*gs*/)
{}
void GameScene::render(Renderer& renderer)
{
    springBackground.draw(renderer.get_renderer());
    renderer.draw_text("HEWWO?????", Vec2(0.0f, 0.0f));
}