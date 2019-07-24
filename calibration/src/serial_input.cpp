#include "serial_input.h"

#include <algorithm>
#include <vector>
#include <stdint.h>
#include <fstream>

#define MAX_DATAPOINTS 300000

unsigned int myo1_datapoints[MAX_DATAPOINTS] = {};
unsigned int myo2_datapoints[MAX_DATAPOINTS] = {};
unsigned int eeg_datapoints[MAX_DATAPOINTS] = {};

int index = 0;
bool filledArray = false;

void SerialInput::add_datapoint(unsigned int myo1V, unsigned int myo2V, unsigned int eegV)
{
    myo1_datapoints[index] = myo1V;
    myo2_datapoints[index] = myo2V;
    eeg_datapoints[index] = eegV;
    if (++index == MAX_DATAPOINTS)
    {
        filledArray = true;
        index = 0;
    }
}

void analyze(unsigned int* data, int arrlen, std::ofstream& outfile)
{
    std::vector<unsigned int> unique_values(data, data + arrlen);
    std::sort(unique_values.begin(), unique_values.end());
    unique_values.erase(std::unique(unique_values.begin(), unique_values.end()), unique_values.end());

    int midpoint = (int)unique_values.size() / 2;
    unsigned int median = unique_values[midpoint];

    printf("Median: %u\n", median);

    double lowAvg = 0.0;
    double highAvg = 0.0;
    int lowCount = 0;
    int highCount = 0;
    for (int i = 0; i < arrlen; ++i)
    {
        if (data[i] < median)
        {
            lowAvg += (double)data[i];
            lowCount++;
        }
        else
        {
            highAvg += (double)data[i];
            highCount++;
        }
    }
    if (lowCount > 0)
        lowAvg /= lowCount;
    if (highCount > 0)
        highAvg /= highCount;

    printf("Low average: %.2f\nHigh average: %.2f\n", lowAvg, highAvg);
    outfile << lowAvg << ":" << highAvg << "\n";

    double lowstddev = 0.0;
    double highstddev = 0.0;
    for (int i = 0; i < arrlen; ++i)
    {
        if (data[i] < median)
        {
            double t = (double)data[i] - lowAvg;
            lowstddev += t * t;
        }
        else
        {
            double t = (double)data[i] - highAvg;
            highstddev += t * t;
        }
    }
    if (lowCount > 0)
        lowstddev /= lowCount;
    if (highCount > 0)
        highstddev /= highCount;

    lowstddev = sqrt(lowstddev);
    highstddev = sqrt(highstddev);

    printf("Low standard deviation: %.2f\nHigh standard deviation: %.2f\n", lowstddev, highstddev);
    outfile << lowstddev << ":" << highstddev << "\n";
}

void SerialInput::write_thresholds()
{
    int arrlen = filledArray ? MAX_DATAPOINTS : index;

    std::ofstream outfile("./thresholds.txt");

    printf("MYO 1:\n");
    analyze(myo1_datapoints, arrlen, outfile);
    printf("MYO 2:\n");
    analyze(myo2_datapoints, arrlen, outfile);
    printf("EEG:\n");
    analyze(eeg_datapoints, arrlen, outfile);
}