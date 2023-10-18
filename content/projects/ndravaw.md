
---
title: "National Day of Remembrance and Action on Violence Against Women 2022"
summary: National Day of Remembrance and Action on Violence Against Women Blue and Gold committee build
description: National Day of Remembrance and Action on Violence Against Women Blue and Gold committee build
tags: ["LEDs", "Activism", "Raspberry Pi"]
---

# The event

The National Day of Remembrance and Action on Violence Against Women is a solemn day commemorating the 1989 École Polytechnique massacre, where women engineers were brutally targeted. Each year, our Blue and Gold committee at U of T engineering crafts a memorial. This time, we built a platform with 14 acrylic cutouts, each inscribed with the name of a woman killed, illuminated by LEDs that reacted to pressure plates.
![build](/images/NDRAVAW/build.jpeg)

## What I did

I took on the task of working on the LED display and the interactive elements.

### LEDs

We settled on using WS2812B addressable RGB strips for lighting. I handled the control hardware design and wiring, while my partner took care of power distribution. The lights were managed by a single ESP32 board running [WLED](https://github.com/Aircoookie/WLED), with a Raspberry Pi sending over the necessary commands.

### Pressure Plates

Creating the pressure plates was a fun challenge. With budget limitations, I went for a simple design using cardboard and aluminum foil. The pressure plates, when stepped on, would close a circuit. These were wired to a Raspberry Pi that took care of the logic part, signaling the ESP32 to control the LEDs based on the pressure plate activity.

Despite the budget constraints and a pressing deadline, it was a rewarding experience seeing everything come together, especially knowing the emotional resonance of the memorial.
![pressure plate](/images/NDRAVAW/pressure_plate.jpg)

