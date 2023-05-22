# Lentasmo

A simple API for remotely turning PC screen on or off. It simulates Tasmota API.

## Small history

I use my laptop screen to monitor my 3d printer progress. However, when the printer is not doing anything I want the laptop's screen to be turned off. Mainsail  exposes controls for configured Tasmota devices and that fact is used for turning the laptop's screen on and off. I use Atom socket plugs for the printer and the heated bed so it was natural for me to follow with a similar approach for the laptop's screen.

## .env file

.env file is required for the API to work. You need to set three environmental variables:
- TASMOTA_USER
- TASMOTA_PASSWORD
- DISPLAY (if there is only one screen it default to ":0")