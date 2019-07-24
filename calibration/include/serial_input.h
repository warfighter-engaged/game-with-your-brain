#ifndef WARFIGHTER__SERIAL_INPUT_H__
#define WARFIGHTER__SERIAL_INPUT_H__

#include <memory>
#include <sstream>
#include <string>

#include "input.h"
#include "serial.h"

struct SerialInput
{
    bool myo1_down = false;
    bool myo2_down = false;
    bool eeg_down = false;

    bool passthrough = false;

    std::unique_ptr<Command> myo1;
    std::unique_ptr<Command> myo2;
    std::unique_ptr<Command> eeg;

    SerialPort connection;

    float myo1Threshold = 0.0f;
    float myo2Threshold = 0.0f;
    float eegThreshold = 0.0f;

    SerialInput(GameState* gs)
        : myo1(std::make_unique<WalkLeftCommand>(gs))
        , myo2(std::make_unique<WalkRightCommand>(gs))
        , eeg(std::make_unique<JumpCommand>(gs))
        , connection(SerialPort("/dev/ttyWF1", 115200))
    {}

    void poll_data()
    {
        int available_chars = connection.dataAvailable();
        std::string input;
        input.reserve(available_chars);
        for (int i = 0; i < available_chars; ++i)
        {
            int res = connection.getchar();
            if (res == -1)
            {
                printf("Error getting character from serial port\n");
            }
            else
            {
                input.push_back((char)res);
            }
        }
        std::istringstream iss(input);

        float myo1Val = 0.0f;
        float myo2Val = 0.0f;
        float eegVal = 0.0f;

        iss >> myo1Val;
        iss >> myo2Val;
        iss >> eegVal;

        printf("INPUT: %s\n", input.c_str());
        printf("MYO1: %.2f\nMYO2: %.2f\nEEG: %.2f", myo1Val, myo2Val, eegVal);

        if (myo1Val > myo1Threshold && !myo1_down)
        {
            myo1_down = true;
            myo1->execute();
        }
        else if (myo1Val < myo1Threshold && myo1_down)
        {
            myo1_down = false;
        }

        if (myo2Val > myo2Threshold && !myo2_down)
        {
            myo2_down = true;
            myo2->execute();
        }
        else if (myo2Val < myo2Threshold && myo2_down)
        {
            myo2_down = false;
        }

        if (eegVal > eegThreshold && !eeg_down)
        {
            eeg_down = true;
            eeg->execute();
        }
        else if (eegVal < eegThreshold && eeg_down)
        {
            eeg_down = false;
        }

        if (passthrough)
        {
            // Parse the data and send it to the XAC
        }
    }

    void retrigger()
    {
        if (myo1->retrigger && myo1_down)
        {
            myo1->execute();
        }
        if (myo2->retrigger && myo2_down)
        {
            myo2->execute();
        }
        if (eeg->retrigger && eeg_down)
        {
            eeg->execute();
        }
    }
};

#endif // WARFIGHTER__SERIAL_INPUT_H__
