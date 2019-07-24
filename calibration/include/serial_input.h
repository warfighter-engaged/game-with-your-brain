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

    std::unique_ptr<Command> myo1;
    std::unique_ptr<Command> myo2;
    std::unique_ptr<Command> eeg;

    SerialPort inputSerial;

    float myo1Threshold = 0.0f;
    float myo2Threshold = 0.0f;
    float eegThreshold = 0.0f;

    SerialInput(GameState* gs)
        : myo1(std::make_unique<WalkLeftCommand>(gs))
        , myo2(std::make_unique<WalkRightCommand>(gs))
        , eeg(std::make_unique<JumpCommand>(gs))
        , inputSerial(SerialPort("/dev/ttyWF1", 115200))
    {}

    void poll_data()
    {
        int available_chars = inputSerial.dataAvailable();
        std::string input;
        input.reserve(available_chars);
        for (int i = 0; i < available_chars; ++i)
        {
            int res = inputSerial.getchar();
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

        unsigned int myo1Val = 0;
        unsigned int myo2Val = 0;
        unsigned int eegVal = 0;

        iss >> myo1Val;
        iss >> myo2Val;
        iss >> eegVal;

        printf("MYO1: %u\nMYO2: %u\nEEG: %u\n", myo1Val, myo2Val, eegVal);

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

        add_datapoint(myo1Val, myo2Val, eegVal);
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

    void write_thresholds();
    void add_datapoint(unsigned int myo1V, unsigned int myo2V, unsigned int eegV);
};

#endif // WARFIGHTER__SERIAL_INPUT_H__
