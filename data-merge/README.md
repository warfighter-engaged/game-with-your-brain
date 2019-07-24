# Data Merge/Output
Gathers the data from the MYO and EEG sensors, runs it through our calibration software and then outputs the results to a controller that will drive inputs into the Xbox Adaptive Controller (XAC).

### Open Issues
* What language are we going to use?

## Springboard
Interfaces between the Raspberry PI and the XAC.

### Data Format
The springboard reads data from send over the serial connection. It expects 5 characters with the following format:
* Characters 0 thru 3 - controls the digital output where '0' is off and '1' is on.
* Character 4 - controls the analog output with a range of '0' through '9'.