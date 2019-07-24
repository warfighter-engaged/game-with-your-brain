#ifndef WARFIGHTER__INPUT_H__
#define WARFIGHTER__INPUT_H__

#include "game_state.h"

class InputManager
{};

class Command
{
    GameState* _gs;

protected:
    virtual void execute(GameState&) = 0;

public:
    virtual ~Command()
    {}
    explicit Command(GameState* gameState)
        : _gs(gameState)
    {}
    void execute()
    {
        execute(*_gs);
    }
};

class JumpCommand : public Command
{
protected:
    void execute(GameState& gs) override
    {
        gs.jump();
    }

public:
    explicit JumpCommand(GameState* gameState)
        : Command(gameState)
    {}
};

class WalkLeftCommand : public Command
{
protected:
    void execute(GameState& gs) override
    {
        gs.walkLeft();
    }

public:
    explicit WalkLeftCommand(GameState* gameState)
        : Command(gameState)
    {}
};

class WalkRightCommand : public Command
{
protected:
    void execute(GameState& gs) override
    {
        gs.walkRight();
    }

public:
    explicit WalkRightCommand(GameState* gameState)
        : Command(gameState)
    {}
};

class NextLineCommand : public Command
{
protected:
    void execute(GameState& gs) override
    {
        gs.nextLine();
    }

public:
    explicit NextLineCommand(GameState* gameState)
        : Command(gameState)
    {}
};

class MenuSelectCommand : public Command
{
protected:
    void execute(GameState& gs) override
    {
        gs.menuSelect();
    }

public:
    explicit MenuSelectCommand(GameState* gameState)
        : Command(gameState)
    {}
};

class ExitCommand : public Command
{
protected:
    void execute(GameState& gs) override
    {
        gs.exit();
    }

public:
    explicit ExitCommand(GameState* gameState)
        : Command(gameState)
    {}
};

#endif // WARFIGHTER__INPUT_H__
