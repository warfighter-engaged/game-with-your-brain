#ifndef WARFIGHTER__VEC2_H__
#define WARFIGHTER__VEC2_H__

#include <math.h>

class Vec2
{
public:
    Vec2(float x, float y)
    {
        e[0] = x;
        e[1] = y;
    }
    inline float x() const
    {
        return e[0];
    }
    inline float y() const
    {
        return e[1];
    }

    inline const Vec2& operator+() const
    {
        return *this;
    }
    inline Vec2 operator-() const
    {
        return Vec2(-e[0], -e[1]);
    }
    inline float operator[](int i) const
    {
        return e[i];
    }
    inline float& operator[](int i)
    {
        return e[i];
    }

    Vec2& operator+=(const Vec2& v2);
    Vec2& operator-=(const Vec2& v2);
    Vec2& operator*=(const Vec2& v2);
    Vec2& operator/=(const Vec2& v2);
    Vec2& operator*=(const float t);
    Vec2& operator/=(const float t);

    inline float length() const
    {
        return sqrtf(e[0] * e[0] + e[1] * e[1]);
    }
    inline float squared_length() const
    {
        return e[0] * e[0] + e[1] * e[1];
    }
    void make_unit_vector();
    Vec2 unit_vector() const;

    float e[2];
};

#endif // WARFIGHTER__VEC2_H__
