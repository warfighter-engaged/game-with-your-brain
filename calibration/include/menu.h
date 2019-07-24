#ifndef WARFIGHTER__MENU_H__
#define WARFIGHTER__MENU_H__

#include <string>
#include <vector>

#include "renderer.h"
#include "vec2.h"

class Menu
{
private:
    std::vector<std::string> _options;
    int _selection;

public:
    Menu(std::vector<std::string> options)
        : _options(options)
        , _selection(0)
    {
        printf("Created menu with size %zu\n", _options.size());
    }

    void next()
    {
        printf("Next!\n");
        _selection++;
        if (_selection >= static_cast<int>(_options.size())) // downcast, but should be ok
        {
            _selection = 0;
        }
    }
    void prev()
    {
        _selection--;
        if (_selection < 0)
        {
            _selection = (int)(_options.size() - 1); // loss of precision ok
        }
    }
    unsigned int selection() const
    {
        return (unsigned int)_selection;
    }

    void draw(Renderer& renderer)
    {
        const float y_offset = 100.0;
        const float x_offset = 220.0;
        const int size = (int)_options.size(); // downcast, but should be ok
        for (int index = 0; index < size; ++index)
        {
            const float y_pos = (index * 40.0f) + y_offset;
            const float x_pos = x_offset;
            const float x_pos_selected = x_offset + 40.0f;
            if (index != _selection)
            {
                renderer.draw_text(_options[index].c_str(), Vec2(x_pos, y_pos), 255, 255, 255);
            }
            else
            {
                renderer.draw_text(("> " + _options[index]).c_str(), Vec2(x_pos_selected, y_pos), 255, 0, 0);
            }
        }
    }
};

#endif // WARFIGHTER__MENU_H__
