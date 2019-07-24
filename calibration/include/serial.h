#ifndef WARFIGHTER__SERIAL_H__
#define WARFIGHTER__SERIAL_H__

#include <stdio.h>
#include <stdarg.h>

#ifndef _WIN32
#include <wiringSerial.h>
#endif

const char text[] = "200 300 400";
const int texLen = 12;

class SerialPort
{
private:
    int fd;
#ifdef _WIN32
    int index;
#endif

public:
    SerialPort(const char* device, int baud)
    {
#ifdef _WIN32
        printf("Baud: %d\nDevice: %s", baud, device);
        fd = 0;
        index = 0;
#else
        fd = serialOpen(device, baud);
#endif
    }

    ~SerialPort()
    {
#ifdef _WIN32
#else
        serialClose(fd);
#endif
    }

    void putchar(unsigned char c)
    {
#ifdef _WIN32
        printf("%c", c);
#else
        serialPutchar(fd, c);
#endif
    }

    void puts(char* s)
    {
#ifdef _WIN32
        printf("%s", s);
#else
        serialPuts(fd, c);
#endif
    }

    int dataAvailable()
    {
#ifdef _WIN32
        return texLen;
#else
        return serialDataAvail(fd);
#endif
    }

    int getchar()
    {
#ifdef _WIN32
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
#ifdef _WIN32
        printf("Flush\n");
#else
        serialFlush(fd);
#endif
    }
};

#endif // WARFIGHTER__SERIAL_H__
