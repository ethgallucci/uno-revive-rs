
<h1 align="center">
    AVR-Garduino
</h1>

<div align="center">
    <img src="/doc/img/nymph.jpeg" width="444" />
</div>

### References

[Arduino Garden Controller](https://practical.engineering/blog/2016/1/20/arduino-garden-controller)

#### Pipeline

[Data Pipeline Schemas](https://github.com/ethgallucci/garduino-rs/blob/main/doc/PIPELINES.md)

# Project Schema
<aside>
💡 Boards A and B can optimally be condensed into one board that carries out both: sensor capture & data exfil to SD (or ethernet, or analog out etc.). Assume boards A and B are one and I/O is condensed into the fewest connections possible. So board ‘A’ would take sensor data as input and split output into two channels. One going to the pins controlling data exfil (SD/ethernet/analog), and one going to the two m2560 displays via analog out.

</aside>

- 1-2 Uno R3 m328p
    - Soil moisture sensor + sunlight sensor: Board A (sensors)
    - I/O shield + SD card module for logging data: Board B (logger board)
    - Optimal: can all be condensed into one board
- 2 Uno R3 m2560
    - OLED display shields for values on board near garden
    - Boards C & D (displays)

# R3 m328p A (sensors)

Written in Rust

Controls all sensors, captures soil moisture and sunlight inputs, batches them, and outputs them to the logger board. 

- Soil Moisture sensors:
    - [https://vetco.net/products/arduino-soil-moisture-sensor-d53](https://vetco.net/products/arduino-soil-moisture-sensor-d53)
- Sunlight sensors:
    - [https://www.robotshop.com/en/grove-sunlight-sensor.html](https://www.robotshop.com/en/grove-sunlight-sensor.html)

Connections:

- In: Soil & sunlight sensors
- Out-Channel 1: Analog out to m2560 OLED displays
- Out-Channel 2: SD/ethernet/analog data exfil (logger). See [Data Pipeline Schemas](Garden%201d7ed/Data%20Pipel%20df07a.md)
- Good SD expansion modules:
    - [https://www.adafruit.com/product/254?gclid=EAIaIQobChMI7YW4hYn29gIVFBvnCh0aVA-PEAQYAiABEgJgzvD_BwE](https://www.adafruit.com/product/254?gclid=EAIaIQobChMI7YW4hYn29gIVFBvnCh0aVA-PEAQYAiABEgJgzvD_BwE)

# Uno R3 m2560 (displays)

Written in Rust

Two equipped with OLED displays to monitor garden directly from enclosure.

## Code
Each board runs unique code, repo is seperated by which board the code'll run on.
The m328p board is used for capturing input from sensors and logging it (writing data to an SD), and the 
m2560 board is simply used for displaying real-time data on OLED display shields so the monitoring station looks cool!
