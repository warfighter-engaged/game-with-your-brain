#ifndef WARFIGHTER__GAME_STATE_H__
#define WARFIGHTER__GAME_STATE_H__

class GameState
{
public:
    void jump();
    void walkLeft();
    void walkRight();
    void menuSelect();
    void exit();

    bool shouldExit = false;
    int x = 0;
    int y = 0;
};

#endif // WARFIGHTER__GAME_STATE_H__
