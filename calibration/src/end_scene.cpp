#include "end_scene.h"

void EndScene::update(float deltaTime, GameState* /*gs*/)
{
}

void EndScene::render(Renderer& renderer)
{
    winterBackground.draw(renderer.get_renderer());
    SDL_SetRenderDrawColor(renderer.get_renderer(), 255, 255, 255, 255);
    SDL_RenderFillRect(renderer.get_renderer(), &textbox);
    nextLine(renderer);
}

void EndScene::nextLine(Renderer& renderer)
{
    renderer.draw_text_wrapped("Congratulations! You're ready to play!", Vec2(5.0f, 405.0f), 690); // width = 700, 5 px padding on each side
}