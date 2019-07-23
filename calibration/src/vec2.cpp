#include "vec2.h"
#include <iostream>

inline std::istream &operator>>(std::istream &is, Vec2 &t)
{
    is >> t.e[0] >> t.e[1];
    return is;
}

inline std::ostream &operator<<(std::ostream &os, const Vec2 &t)
{
    os << t.e[0] << " " << t.e[1];
    return os;
}

inline void Vec2::make_unit_vector()
{
    float sqr_len = squared_length();
    if (sqr_len == 0.0)
    {
        return;
    }
    float k = 1.0f / sqrtf(sqr_len);
    e[0] *= k;
    e[1] *= k;
}

inline Vec2 operator+(const Vec2 &v1, const Vec2 &v2)
{
    return Vec2(v1.e[0] + v2.e[0], v1.e[1] + v2.e[1]);
}

inline Vec2 operator-(const Vec2 &v1, const Vec2 &v2)
{
    return Vec2(v1.e[0] - v2.e[0], v1.e[1] - v2.e[1]);
}

inline Vec2 operator*(const Vec2 &v1, const Vec2 &v2)
{
    return Vec2(v1.e[0] * v2.e[0], v1.e[1] * v2.e[1]);
}

inline Vec2 operator/(const Vec2 &v1, const Vec2 &v2)
{
    return Vec2(v1.e[0] / v2.e[0], v1.e[1] / v2.e[1]);
}

inline Vec2 operator*(float t, const Vec2 &v)
{
    return Vec2(t * v.e[0], t * v.e[1]);
}

inline Vec2 operator*(const Vec2 &v, float t)
{
    return Vec2(t * v.e[0], t * v.e[1]);
}

inline Vec2 operator/(const Vec2 &v, float t)
{
    return Vec2(v.e[0] / t, v.e[1] / t);
}

inline float dot(const Vec2 &v1, const Vec2 &v2)
{
    return v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1];
}

inline Vec2 &Vec2::operator+=(const Vec2 &v)
{
    e[0] += v.e[0];
    e[1] += v.e[1];
}

inline Vec2 &Vec2::operator-=(const Vec2 &v)
{
    e[0] -= v.e[0];
    e[1] -= v.e[1];
}

inline Vec2 &Vec2::operator*=(const Vec2 &v)
{
    e[0] *= v.e[0];
    e[1] *= v.e[1];
}

inline Vec2 &Vec2::operator/=(const Vec2 &v)
{
    e[0] /= v.e[0];
    e[1] /= v.e[1];
}

inline Vec2 &Vec2::operator*=(float t)
{
    e[0] += t;
    e[1] += t;
}

inline Vec2 &Vec2::operator/=(float t)
{
    e[0] /= t;
    e[1] /= t;
}

inline Vec2 Vec2::unit_vector() const
{
    float sqr_len = squared_length();
    if (sqr_len == 0.0)
    {
        return Vec2(this->e[0], this->e[1]);
    }
    float k = 1.0f / sqrtf(sqr_len);
    return Vec2(this->e[0] * k, this->e[1] * k);
}