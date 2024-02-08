/*
 * Created by ArduinoGetStarted.com
 * Tutorial page: https://arduinogetstarted.com/tutorials/arduino-rfid-nfc-door-lock-system
 *
 * Code edited by Adam, to suit our smart locker
 */

#include <SPI.h>
#include <MFRC522.h>

#define SS_PIN 10
#define RST_PIN 5
#define RELAY_PIN A1 // the Analogue pin that connects to relay

MFRC522 rfid(SS_PIN, RST_PIN);

byte keyTagUID[4] = {0x4B, 0xC4, 0x19, 0x04};

void setup() {
  Serial.begin(9600);
  SPI.begin(); // initialize the SPI bus
  rfid.PCD_Init(); // initialize the MFRC522 (RFID Module)
  pinMode(RELAY_PIN, OUTPUT); // initialize pin as an output
  digitalWrite(RELAY_PIN, LOW); // lock the door

  Serial.println("Tap RFID/NFC Tag on reader");
}

void loop() {
  if (rfid.PICC_IsNewCardPresent()) { // new tag is available
    if (rfid.PICC_ReadCardSerial()) { // new UID has been read
      MFRC522::PICC_Type piccType = rfid.PICC_GetType(rfid.uid.sak);

      if (rfid.uid.uidByte[0] == keyTagUID[0] &&
          rfid.uid.uidByte[1] == keyTagUID[1] &&
          rfid.uid.uidByte[2] == keyTagUID[2] &&
          rfid.uid.uidByte[3] == keyTagUID[3] ) {
        Serial.println("Access is granted");
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

      delay(2000); // delay before the system can read another RFID card
    }
  }
}