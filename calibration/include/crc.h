#ifndef WARFIGHTER__CRC_H__
#define WARFIGHTER__CRC_H__

// Handles string hashing using the CRC-32 algorithm

#include <stdint.h>
#include <string.h>

typedef uint32_t CRC;

constexpr uint32_t Polynomial = 0xEDB88320;

int constexpr get_len(const char *str)
{
    return *str ? 1 + get_len(str + 1) : 0;
}

constexpr CRC crc32_bitwise(const char *data, CRC previousCrc = 0)
{
    CRC crc = ~previousCrc;
    unsigned char *current = (unsigned char *)data;
    size_t length = get_len(data);
    while (length--)
    {
        crc ^= *current++;
        for (unsigned int j = 0; j < 8; j++)
        {
            crc = (crc >> 1) ^ (-int(crc & 1) & Polynomial);
        }
    }
    return ~crc;
}

#define WFID(s) crc32_bitwise(s)

#endif // WARFIGHTER__CRC_H__
