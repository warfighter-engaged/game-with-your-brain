#include <Mindwave.h>

int ledPin = 13;
Mindwave mindwave;

//---------------------------------------------------------------------------------

void setup(){ 
  // start mindwave and tell it to debug
  // this will automatically open the Serial port at 57600 baud
  mindwave.setup();
  mindwave.setDebug(false);

  pinMode(ledPin, OUTPUT);
} 

void loop(){ 
  // get new brainwave data
  // the line below prints attention values
  mindwave.update();

  // light led if signal quality is good
  if (mindwave.getQuality() == 200) {
    digitalWrite(ledPin, HIGH);
  }
  else {
    digitalWrite(ledPin, LOW);
  }
} 
