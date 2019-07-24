const int maxDigitalButtons = 4;
const int baseDigitalPin = 3;
const int analogPin = 9;

bool hasNewData = false;
bool pinValue[maxDigitalButtons];

int analogValue = 0;

// Runs once when you press reset or power the board
void setup()
{
  Serial.begin(115200);
  while (!Serial) {
    ; // wait for serial port to connect.
  }

  memset(pinValue, 0, sizeof(pinValue));

  // Initialize digital pin LED_BUILTIN as an output
  pinMode(LED_BUILTIN, OUTPUT);  

  // Initialize digital pins for our buttons as output
  for (int i = 0; i < maxDigitalButtons; ++i)
  {
    pinMode(baseDigitalPin + i, OUTPUT);
  }

  pinMode(9, OUTPUT);
}

// Runs over and over again forever
void loop()
{
  readInput();
  setOutput();
}

void readInput()
{
  int availableBytes = Serial.available();
  if (availableBytes >= (maxDigitalButtons + 1))
  {
    uint8_t buffer[maxDigitalButtons + 1] = {0};

    buffer[0] = Serial.read() - '0';
    buffer[1] = Serial.read() - '0';
    buffer[2] = Serial.read() - '0';
    buffer[3] = Serial.read() - '0';
    buffer[4] = Serial.read() - '0';

    // Clear out anything else in the rx queue.
    while (Serial.available())
    {
      Serial.read();
    }

    for (int i = 0; i < maxDigitalButtons; ++i)
    {
      pinValue[i] = buffer[i] > 0;
    }

    analogValue = 255 * buffer[4] / 9;

    hasNewData = true;
  }
}

void setOutput()
{
  if (hasNewData)
  {
    // Update the digital pins, which are packed in the front
    // of the values array.
    for (int i = 0; i < maxDigitalButtons; ++i)
    {
      digitalWrite(baseDigitalPin + i, pinValue[i] ? HIGH : LOW);
    }

    // Update the analog pin, which is the last value in the
    // array.
    analogWrite(analogPin, analogValue);

    // We've processed the new data, so clear the flag.
    hasNewData = false;
  }
}
