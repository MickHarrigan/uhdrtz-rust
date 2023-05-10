# Introduction

## Purpose and Scope
This section provides a brief description of the Systems Design Document’s purpose and scope.

## Project Executive Summary
This section provides a description of the project from a management perspective and an overview of the framework within which the conceptual system design was prepared.  If appropriate, include the information discussed in the subsequent sections in the summary.

### System Overview
This section describes the system in narrative form using non-technical terms.  It should provide a high-level system architecture diagram showing a subsystem breakout of the system, if applicable.  The high-level system architecture or subsystem diagrams should, if applicable, show interfaces to external systems.  Supply a high-level context diagram for the system and subsystems, if applicable.

### Design Constraints
This section describes any constraints in the system design (reference any trade-off analyses conducted such, as resource use versus productivity, or conflicts with other systems) and includes any assumptions made by the project team in developing the system design.

### Future Contingencies
Test

## Document Organization
Test

## Points of Contact

**Micheal (Mick) Harrigan**
- **Personal Email:** mharrigan328@gmail.com
- **School Email:** mharrig1@umbc.edu

**Daniel (Dan) Cleaver**
- **Personal Email:**
- **School Email:**

**Christian Lostoski**
- **Personal Email:** christian.lostoski@gmail.com
- **School Email:** christl1@umbc.edu

**Nomso Ashiogwu**
- **Personal Email:**
- **School Email:** cashiog1@umbc.edu

## Project References
Test

## Glossary
Test

# System Architecture
Test

## System Hardware Architecture

This section will cover all of the hardware used in the UHDRTZ system. It will also describe connectios between hardware components.

### 4K USB-C Camera

The 4K USB-C Camera is a small, roughly 1.5" x 1.5" x 1" camera that connects to the Mini PC via a USB-C (male) to USB-A (male) cable. The camera is powered by the Mini PC and does not require any additional power. 

### Mini PC

The Mini PC is a small form factor personal computer with a capable gpu and cpu. It will be powered by a AC adapter and will be connected to a 4K projector via HDMI. The Mini PC will be running a Linux operating system and will be running the UHDRTZ application. The PC will be connected to the USB Camera via a USB-C to USB-A cable. It will also be connected to whatever monitor is chosen via a HDMI cable.
### Crank Housing

The crank housing is an aluminum box that will hold the Arduino and Rotary Encoder. A small wheel is attatched to the outside of the housing via the rotary encoder. There are holes in the base plate and the bottom of the box to allow a Micro-USB cable to be threaded through.
#### Arduino

The Arduino is a standard Arduino NANO BLE (Bluetooth). It will send signal via Bluetooth to the Mini PC. It will be connected to the Rotary Encoder via a 4 1-pin cables. Refer to the Hardware Description section to see the connections. It will be powered by a Micro-USB cable.

#### Rotary Encoder

The Rotary Endocer will be a standar Arduino Kit rotary encoder. It will be connected and powered by the Arduino via 4 1 pin cables. Refer to the Hardware Description section to see the connections.


## System Software Architecture
Test

## Internal Communications Architecture
Test

# File and Database Design
Test

## Database Management System Files
Test

## Non-Database Management System Files
Test

# Human-Machine Interface

This section will cover all of the user interactions with the UHDRTZ system. It will also cover the user interface and how to use it.

## Operational Scenario(s)
This will be an in-depth instructional manual for getting the High- Definition Real Time Zoetrope system up and running.

### Whats Included & Whats Needed
The following list are all item that will be included in the UHDRTZ kit. This section will also cover what will be needed during the installation process.
- INCLUDED
    - Mini PC
    - AC Power Cable
    - 4K USB-C Camera
    - USB-A to USB-C Cable
    - Micro USB to USB-A Cable
    - Crank Housing
        - Arduino
        - Rotary Encoder
- Suggested items for setup
    - Level
    - Tape
    - USB Keyboard & Mouse

### Where to Start
This section will cover the setup of the physical system. The installation of the artwork is assumed.

- Place the camera so that is points towards the center of the artwork and is directly in line with the center of the artwork.
    - If you are pointing the camera down to look at the artwork on the floor, a level would be useful. 
- Mount the Mini PC close to the camera. 
    - Attach the camera to the Mini PC using the USB-C to USB-A cable. 
    - Plug the AC Power Cable into the mini and connect it to a standard outlet. 
- Set up the crank housing on the floor where users can reach it. 
    - Connect the Micro USB to USB-A Cable to the Arduino within the crank housing and string the cable through the whole in the bottom of the crank housing. 
    - Connect that cable to power 
    - (**Note:** best option is to use a USB-A to wall socket brick and connect the Arduino directly to power)
- Plug in a USB Keyboard & Mouse into the Mini PC. 
- Connect the Mini PC to an external monitor (4K Projector). 
- Power on the PC. 


### Starting The Program

Once you have a physical components of the system set up, you will want to power on the Mini PC. There will be two sign-in options. As a user, you will want to click on the UHDRTZ profile. Once logged in you will want to start the UHDRTZ application. You can do this by either pressing the Windows key on your keyboard or by moving the cursor to the upper left hand corner of the screen. Once you have done this, the UHDRTZ application should appear on the left side of the screen. Double click the application to start it. Once you have opened the application, you will see the following startup screen. 

![alt text](https://github.com/clostoski/uhdrtz-images/blob/main/Startup_screen.png?raw=true)

There are six sections total to the startup screen.

![alt text](https://github.com/clostoski/uhdrtz-images/blob/main/startup_options.png?raw=true)

1. Camera
    - This drop-down menu allows users to select from the available connected cameras.
2. Quality
    - This drop-down menu allows users to select their desired screen resolution and frame rate.
    - i.e. 1080p at 60 FPS
3. Crank
    - This section shows whether the crank box Arduino is connected to the Mini PC through Bluetooth or not.
    - When it is not connected there will be a spinning loading symbol
    - If the Arduino fails to connect automatically check that the Arduino if powered on. If it is powered on and not connecting, press the small button on the Arduino once to reset it.
3. Audio
    - This drop-down allows you to select from all loaded audio files. The UHDRTZ comes with one pre-loaded audio file.
    - If users wish to add a different audio file, press the **Open Audio Location** button to the right of the audio section.
4. Slices
    - This text box allows users to input the number of *slices* the displayed piece of art has. This will help determine the rotation speed and frame-rate of the program.
5. Continue
    - Once all settings are set to the users satisfaction, press the continue button to launch the full application
    - **Note:** The Continue button will not be press-able until the crank Arduino is connected through Bluetooth.

## Using the Program

![alt text](https://github.com/clostoski/uhdrtz-images/blob/main/Application%20screen.png?raw=true)

At this point, the screen will be displaying a circular view of the camera's input. To open up the settings, press **SPACE**. This will open up the user interface as well as show the crosshairs. The user can utilize the crosshairs to line up the camera perfectly with the art so that the center of rotation of the art is the same as the cameras. Upon opening the settings, the user will also have access to several sub menus. 

1. Effects
    - The effects menu will allow the user to change the in camera settings. The settings are as follows:
        - Brightness
        - Contrast
        - Saturation
        - Gamma
        - White Balance

![alt text](https://github.com/clostoski/uhdrtz-images/blob/main/Effects.png?raw=true)

2. Volume
    - This will allow the user to adjust the volume of the audio withing the program.

![alt text](https://github.com/clostoski/uhdrtz-images/blob/main/volume.png?raw=true)

3. Presets
    - This menu allows the user to select from several preset location options
        - Re-Center
            - This will place the image back into its default configuration
        - Semi-Circle
            - This will place the image so that only the top half is visible
        - Right
            - This will place the image on the right half of the screen and only display one quarter of the image
        - Left
            - This will place the image on the left half of the screen and only display one quarter of the image
    - **Note:** The user can change the location and size of the image at any time by using the arrow keys (for position) and using PageUp/PageDown (for size)

![alt text](https://github.com/clostoski/uhdrtz-images/blob/main/presets.png?raw=true)

## Inputs
Test

## Outputs
Test

# Detailed Design
Test

## Hardware Detailed Design

This file will cover all hardware components included in the UHDRTZ kit. It will also cover basic setup and descriptions of the components for a more in depth explanation of how to use them. The components covered are as followed.

- 4K USB-C Camera
- Mini PC
- Crank Housing
  - Arduino
  - Rotary Encoder

### 4K USB-C Camera

The Camera used for this project will be the Econ Systems See3CAM_CU135. This camera has one USB-C 3.0 port. It has a 13MP fixed lens with good low light performance and iHDR support. This camera was chosen for this project due to its small form factor and easy setup.

- **Key Features**
  - **Frame Rate:**
    - Full HD @ 60 fps, 4k @ 30 fps & VGA @ 120 fps
    - Up to 816 fps for Custom ROI*
      - Refer to the datasheet for complete frame rate details. <https://www.e-consystems.com/4k-usb-camera.asp>
    - Output format: Uncompressed UYVY and Compressed MJPEG
    - Supported OS: Windows, Linux, Android ** and MAC ***
    - iHDR support
    - Unique ID for each camera
  - **Interface:**
    - USB 3.1 Gen 1
    - Type-C reversible interface connector
    - UVC compliant - no additional drivers required
    - Backward compatible with USB 2.0 hosts
  - **Module Features:**
    - Sensor: AR1334 from onsemi&reg;
    - Focus Type: Fixed focus
    - Sensor Resolution: 13MP
    - Chroma: Color
    - Shutter Type: Electronic Rolling Shutter with global reset mode #
    - Optical Formal: 1/3.2"
    - Output Format: Uncompressed UYVY and Compressed MJPEG
    - Pixel Size: 1.1 $\mu m$ x 1.1 $\mu m$
    - Sensor Active Area: 4208 (H) x 3210 (V)
    - Array Size: 4280 x 3120 Pixel
    - Responsivity: 4700 e-/lux-sec
    - SNR: 37 dB
    - Dynamic Range: 69 dB (nice)
    - FOV: 67&deg;(D), 56&deg;(H), 43&deg;(V) (with the lens provided by e-con)
  - **Electrical and mechanical:**
    - Operating Voltage: 5 v +/- 5%
    - Operating Temperature Range: Without Enclosure: -30&deg;C to 70&deg;C
    - Power Requirements: Max: 1.99W, Min: 1.04W
    - Size in mm (l x b h):
      - Without Lens: 35.3 x 35.3 x 29 mm
    - Board Weight:
      - Without Lens: 55.5 Grams
      - With Lens: 63.5 Grams
  - Miscellaneous:
    - Compliance: FCC, RoHS

*- Not supported by default. Requires firmware customization for higher frame rates with Custom ROI resolutions.

**- Customers interested to work on Android would require e-con SDK

***- For MAC OS support please contact camerasolutions@e-consystems.com

#- Rolling shutter is supported in the default firmware. Customized firmware/hardware is required to use a rolling shutter with Global reset mode.

### Mini PC

The Mini PC used in this project is the GMKtec Intel 11th i5 1135G7 Mini PC--NucBox 2 Plus.

- OS: Debian 11
- CPU: Intel 11th i5 1135g7
- Graphics: Intel&reg; Iris&reg; Xe Graphics
- RAM: 16GB DDR4 3200 MHz
- Memory: 512GB NVMe SSD
- Wi-Fi: Wi-Fi 6, BT 5.2
- Ports:
  - 1x Type-C Thunderbolt 4
  - 2x HDMI 2.0 (4k@60Hz)
  - 4x USB-A 3.2
  - 1x RJ45 Ethernet Port

### Crank Housing
The crank housing will hold the Arduino and rotary encoder. A wheel will be attached to the rotary encoder to allow for the user to turn the crank. The Arduino will be used to read the encoder and send the data to the mini pc.

- Arduino
  - Arduino Nano 33 BLE
  - Micro-controller: nRF52840
  - Operating Voltage: 3.3V
  - Input Voltage (recommended): 7-12V
  - Input Voltage (limit): 6-20V
  - DC Current per I/O Pins: 15 mA
  - Clock Speed: 64 MHz
  - CPU Flash Memory: 1MB (nRF52840)
  - SRAM: 256KB (nRF52840)
  - EEPROM: none
  - Digital I/O Pins: 14
  - PWM Pins: all digital pins
  - UART: 1
  - SPI: 1
  - I2C: 1
  - Analog Input Pins: 8 (ADC 12 bit 200 ksamples)
  - Analog Output Pins: Only Through PWM (no DAC)
  - External Interrupts: all digital pins
  - LED_BUILTIN: 13
  - USB: Native in the nRF52840 Processor
  - Length: 45 mm
  - WidthL 18 mm
  - Weight: 5 gr (with headers)
- Rotary Encoder
  - Model: KY-040
  - Type: Incremental Rotary Encoder
  - Cycles per revolution (CPR): 20
  - Working voltage: 0 - 5V
  - Material: PCB + Brass
  Dimensions: 32 x 19 x 30 mm

In the case that the Arduino becomes detached from the Rotary encoder, please refer to the following diagram for re-connection. 

![alt text](https://github.com/clostoski/uhdrtz-images/blob/main/rotaryencoder_arduino.png?raw=true)

## Software Detailed Desgin
Test

## Internal Communications Detailed Design
Test

# External Interfaces
Test

## Interface Architecture
Test

## Interface Detailed Design
Test

# System Integrity Controls
The system does not use any information that could affect the conduct of state programs or the privacy to which individuals are entitled. Thus, this section is not applicable.

# Appendices

# Test

## Test with vanilla md image
![something](./assets/proj4_02.png)

## Test of raw html figure
```{=html5}
<figure>
    <img src=./assets/2021-08-23-laberge-xmas-original.png alt="laberge xmas" width=300, height=auto />
    <figcaption><b>Figure 1:</b> A festive guy</figcaption>
</figure>
```
