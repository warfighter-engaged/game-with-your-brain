#ifndef WARFIGHTER__SERIAL_H__
#define WARFIGHTER__SERIAL_H__

#include <stdio.h>
#include <stdarg.h>

#if !defined(_WIN32) && !defined(__APPLE__)
#include <wiringSerial.h>
#endif

const char text[] = "200 300 400";
const int texLen = 12;

class SerialPort
{
private:
    int fd;
#if defined(_WIN32) || defined(__APPLE__)
    int index;
#endif

public:
    SerialPort(const char* device, int baud)
    {
#if defined(_WIN32) || defined(__APPLE__)
        printf("Baud: %d\nDevice: %s", baud, device);
        fd = 0;
        index = 0;
#else
        fd = serialOpen(device, baud);
#endif
    }

    ~SerialPort()
    {
#if defined(_WIN32) || defined(__APPLE__)
#else
        serialClose(fd);
#endif
    }

    void putchar(unsigned char c)
    {
#if defined(_WIN32) || defined(__APPLE__)
        printf("%c", c);
#else
        serialPutchar(fd, c);
#endif
    }

    void puts(char* s)
    {
#if defined(_WIN32) || defined(__APPLE__)
        printf("%s", s);
#else
        serialPuts(fd, c);
#endif
    }

    int dataAvailable()
    {
#if defined(_WIN32) || defined(__APPLE__)
        return texLen;
#else
        return serialDataAvail(fd);
#endif
    }

    int getchar()
    {
#if defined(_WIN32) || defined(__APPLE__)
        int res = (int)text[index++];
        if (index == texLen)
        {
            index = 0;
        }
        return res;
#else
        return serialGetchar(fd);
#endif
    }

    void flush()
    {
#if defined(_WIN32) || defined(__APPLE__)
        printf("Flush\n");
#else
        serialFlush(fd);
#endif
    }
};

#endif // WARFIGHTER__SERIAL_H__
