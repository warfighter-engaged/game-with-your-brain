
  int minValue = 200;
  int maxValue = 1000;
  int increment = 15;
  int currentValue = minValue;
  bool increasing = true; 
  
void setup() {
  // put your setup code here, to run once:
  Serial.begin(115200);
  
}

void loop() {
  // put your main code here, to run repeatedly:

 /* if (currentValue > maxValue) {
    increasing = false;
  }
  if (currentValue < minValue) {
    increasing = true;
  }

  if (increasing) {
    currentValue += increment;
  } else {
    currentValue -= increment;
  }*/
  
  
  int readValue = analogRead(A0);
  //int readValue = currentValue;
  Serial.println(readValue, DEC);
  Serial.flush();

  delay(1);
  
  //Serial.write("200,51,30,0.979");
  //Serial.write("57,90");
}
