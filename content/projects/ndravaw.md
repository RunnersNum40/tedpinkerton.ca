---
title: "National Day of Remembrance and Action on Violence Against Women 2022"
summary: National Day of Remembrance and Action on Violence Against Women blue and gold committee build
description: National Day of Remembrance and Action on Violence Against Women blue and gold committee build
tags: ["LEDs", "Activism", "Raspberry Pi"]
---

# The event

The National Day of Remembrance and Action on Violence Against Women is a day to commemorate the anniversary of the 1989 École Polytechnique massacre.
During the massacre women engineers were targeted and killed by a gunman who was "fighting feminism" with 14 fatalities. During the day Canadians observe a moment of scilence.

Each year U of T engineering's Blue and Gold committee builds a memorial. This year's memorial was a platform with 14 acrylic cutouts with the name of each woman killed on one.
The acrylic was lit from underneath by LEDs that reacted to pressure plates under astroturf in front.
![build](/images/NDRAVAW_build.jpeg)

## What I did

I worked on the LED display and the interactive elements.

### LEDs

The LEDs we used were WS2812B addressable RGB strips. I designed and wired the control hardware and a partner designed the power distribution.
I controlled the LEDs from a single ESP32 board running [WLED](https://github.com/Aircoookie/WLED) and sent commands from a Raspberry Pi.
The ESP32 hosted a Wifi network which let us update the display after setup without needing the remove the hardware.

### Pressure plates

I used a common design for cheap pressure plates made from cardboard and aluminium foil.
A cardboard sheet with a cutout is sandwiched between two other sheets with foil on their surface.
When someone steps on the device the sheets contact and close a curcuit.
I added some spacers in the middle to help with durability and reliability since jumping on the early design eventually led to it remaining closed.
![pressure plate](/images/pressure_plate.jpg)
