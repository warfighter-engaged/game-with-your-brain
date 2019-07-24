#include "game_scene.h"

void GameScene::update(float deltaTime, GameState* /*gs*/)
{
    springBackground.position += Vec2(0.0f, 0.0f) * deltaTime;
    //if (springBackground.position.x() > 400.0f)
    //{
    //    springBackground.position[0] = 0.0f;
    //}
    //if (springBackground.position.y() > 400.0f)
    //{
    //    springBackground.position[1] = 0.0f;
    //}
}
void GameScene::render(Renderer& renderer)
{
    springBackground.draw(renderer.get_renderer());
    SDL_SetRenderDrawColor(renderer.get_renderer(), 255, 255, 255, 255);
    SDL_RenderFillRect(renderer.get_renderer(), &textbox);
    
}

void GameScene::nextLine()
{
	// how do we pass renderer into here?
    printf("hello");
    
	//render.draw_text();
}