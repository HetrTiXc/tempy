/*
  Web client

 This sketch connects to a website (http://www.google.com)
 using a WiFi shield.

 This example is written for a network using WPA encryption. For
 WEP or WPA, change the WiFi.begin() call accordingly.

 This example is written for a network using WPA encryption. For
 WEP or WPA, change the WiFi.begin() call accordingly.

 Circuit:
 * WiFi shield attached

 created 13 July 2010
 by dlf (Metodo2 srl)
 modified 31 May 2012
 by Tom Igoe
 */
//#define WINC1501_RESET_PIN 1 //PIN_PA1
//#define WINC1501_INTN_PIN 22 //PIN_PF2
//#define WINC1501_SPI_CS_PIN -1 //PIN_PA7
//#define WINC1501_CHIP_EN_PIN 23 //PIN_PF3

#include <SPI.h>
#include <WiFi101.h>
#include <ArduinoHttpClient.h>
#include "arduino_secrets.h" 
#include "OneWire.h"
#include "DallasTemperature.h"

///////please enter your sensitive data in the Secret tab/arduino_secrets.h
char ssid[] = SECRET_SSID;        // your network SSID (name)
char pass[] = SECRET_PASS;    // your network password (use for WPA, or use as key for WEP)
int keyIndex = 0;            // your network key Index number (needed only for WEP)

int status = WL_IDLE_STATUS;
// if you don't want to use DNS (and reduce your sketch size)
// use the numeric IP instead of the name for the server:
//IPAddress server(74,125,232,128);  // numeric IP for Google (no DNS)
char serverAddress[] = "10.0.10.7";
int port = 9091;

// Initialize the Ethernet client library
// with the IP address and port of the server
// that you want to connect to (port 80 is default for HTTP):
WiFiClient wifiClient;
HttpClient httpClient = HttpClient(wifiClient, serverAddress, port);

OneWire oneWire(PIN_PD4);

DallasTemperature sensors(&oneWire);

void setup() {
  WiFi.setPins(
    PIN_PA7,
    PIN_PF2,
    PIN_PA1,
    PIN_PF3
  );
  //Initialize serial and wait for port to open:
  Serial2.begin(9600);
  //Serial2.println("Halloooooooo");
  pinConfigure(PIN_PD1, PIN_DIR_OUTPUT, PIN_OUT_LOW);
  while (!Serial2) {
    ; // wait for serial port to connect. Needed for native USB port only
  }
  pinConfigure(PIN_PD2, PIN_DIR_OUTPUT, PIN_OUT_LOW);

  SPI.begin();
  // check for the presence of the shield:
  if (WiFi.status() == WL_NO_SHIELD) {
    Serial2.println("WiFi shield not present");
    // don't continue:
    pinConfigure(PIN_PD0, PIN_DIR_OUTPUT, PIN_OUT_LOW);
    while (true);
  }

  // attempt to connect to WiFi network:
  while (status != WL_CONNECTED) {
    Serial2.print("Attempting to connect to SSID: ");
    Serial2.println(ssid);
    // Connect to WPA/WPA2 network. Change this line if using open or WEP network:
    pinConfigure(PIN_PD3, PIN_DIR_OUTPUT, PIN_OUT_LOW);
    status = WiFi.begin(ssid, pass);

    // wait 10 seconds for connection:
    // delay(10000);
  }
  Serial2.println("Connected to wifi");
  printWiFiStatus();

  // Initiate temp sensor
  sensors.begin();
}

void loop() {
  // Serial2.println("making POST request");
  // String contentType = "application/json";
  // String postData = "{\"time\": \"2024-09-20T19:47:50.52Z\", \"value\": 23.0}";

  // httpClient.post("/writeTimeseriesValue", contentType, postData);

  // read the status code and body of the response
  // int statusCode = httpClient.responseStatusCode();
  //String response = httpClient.responseBody();

  // Serial2.print("Status code: ");
  // Serial2.println(statusCode);
  // Serial2.print("Response: ");
  // Serial2.println(response);

  // Get and print temperature
  sensors.requestTemperatures();
  // Fetch the temperature in degrees Celsius for device index:
  float tempC = sensors.getTempCByIndex(0);
  Serial2.print("Temperature: ");
  Serial2.print(tempC);
  Serial2.println("°C");

  delay(5000);
}


void printWiFiStatus() {
  // print the SSID of the network you're attached to:
  Serial2.print("SSID: ");
  Serial2.println(WiFi.SSID());

  // print your WiFi shield's IP address:
  IPAddress ip = WiFi.localIP();
  Serial2.print("IP Address: ");
  Serial2.println(ip);

  // print the received signal strength:
  long rssi = WiFi.RSSI();
  Serial2.print("signal strength (RSSI):");
  Serial2.print(rssi);
  Serial2.println(" dBm");
}





