const int maxButtons = 4;
const int basePin = 3;
int button = 0;

// Runs once when you press reset or power the board
void setup() {
  // Initialize digital pin LED_BUILTIN as an output
  pinMode(LED_BUILTIN, OUTPUT);  

  // Initialize digital pins for our buttons as output
  for (int i = 0; i < maxButtons; ++i)
  {
    pinMode(basePin + i, OUTPUT);
  }
}

// Runs over and over again forever
void loop() {
  // Turn the button on for 1 second
  digitalWrite(LED_BUILTIN, HIGH);
  digitalWrite(basePin + button % maxButtons, HIGH);
  delay(1000);
  
  // Turn the button off for 1 second
  digitalWrite(LED_BUILTIN, LOW);
  digitalWrite(basePin + button % maxButtons, LOW);
  delay(1000);

  // Advance which button we'll drive on the next iteration
  button++;
}
