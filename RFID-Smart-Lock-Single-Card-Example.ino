 * Created by ArduinoGetStarted.com
 * This example code is in the public domain
 * Tutorial page: https://arduinogetstarted.com/tutorials/arduino-rfid-nfc-door-lock-system
 *
 * Edited by adam-does-coding

#include <SPI.h>
#include <MFRC522.h>

#define SS_PIN 10
#define RST_PIN 9
#define RELAY_PIN A5 // the Arduino pin connects to relay

MFRC522 rfid(SS_PIN, RST_PIN);

byte RFIDTagUID[4] = {0xFF, 0xFF, 0xFF, 0xFF}; // change the FFs to the RFID card UID data

void setup() {
  Serial.begin(9600);
  SPI.begin(); // initialize SPI bus
  rfid.PCD_Init(); // initialize MFRC522
  pinMode(RELAY_PIN, OUTPUT); // initialize pin as an output.
  digitalWrite(RELAY_PIN, LOW); // lock the door

  Serial.println("Tap RFID/NFC Tag on reader");
}

void loop() {
  if (rfid.PICC_IsNewCardPresent()) { // new tag is available
    if (rfid.PICC_ReadCardSerial()) { // NUID has been read
      MFRC522::PICC_Type piccType = rfid.PICC_GetType(rfid.uid.sak);

      if (rfid.uid.uidByte[0] == RFIDTagUID[0] &&
          rfid.uid.uidByte[1] == RFIDTagUID[1] &&
          rfid.uid.uidByte[2] == RFIDTagUID[2] &&
          rfid.uid.uidByte[3] == RFIDTagUID[3] ) {
        Serial.println("Access is granted for first card");
        digitalWrite(RELAY_PIN, HIGH);  // unlock the door for 5 seconds
        delay(5000);
        digitalWrite(RELAY_PIN, LOW); // lock the door
      }
      else      
      {
        Serial.print("Access denied for user with UID:");
        for (int i = 0; i < rfid.uid.size; i++) {
          Serial.print(rfid.uid.uidByte[i] < 0x10 ? " 0" : " ");
          Serial.print(rfid.uid.uidByte[i], HEX);
        }
        Serial.println();
      }

      rfid.PICC_HaltA(); // halt PICC
      rfid.PCD_StopCrypto1(); // stop encryption on PCD
    }
  }
}
