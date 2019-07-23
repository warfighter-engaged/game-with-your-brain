const int maxDigitalButtons = 4;
const int baseDigitalPin = 3;
const int analogPin = 9;

bool hasNewData = false;
bool pinValue[maxDigitalButtons];

// Runs once when you press reset or power the board
void setup()
{
  Serial.begin(9600);

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
  if (availableBytes > 0)
  {
    uint8_t buffer[5] = {0};
    Serial.readBytes(buffer, sizeof(buffer));

    for (int i = 0; i < maxDigitalButtons; ++i)
    {
      pinValue[i] = buffer[i] > 0;
    }

    pinValue[maxDigitalButtons] = buffer[maxDigitalButtons];

    hasNewData = true;
  }
}

void setOutput()
{
  digitalWrite(LED_BUILTIN, LOW);

  if (hasNewData)
  {
    // Update the digital pins, which are packed in the front
    // of the values array.
    for (int i = 0; i < maxDigitalButtons; ++i)
    {
      digitalWrite(baseDigitalPin + i, pinValue[i] > 0 ? HIGH : LOW);
    }

    // Update the analog pin, which is the last value in the
    // array.
    analogWrite(analogPin, pinValue[maxDigitalButtons]);
    
    // Debug LED for when we process data.
    digitalWrite(LED_BUILTIN, HIGH);

    // We've processed the new data, so clear the flag.
    hasNewData = false;
  }
}
