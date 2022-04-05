
<h1 align="center">
    AVR-Garduino
</h1>

<div align="center">
    <img src="/doc/img/nymph.jpeg" width="444" />
</div>

### References

[Arduino Garden Controller](https://practical.engineering/blog/2016/1/20/arduino-garden-controller)


# Components & Controllers

- 1-2 Uno R3 m328p
    - Soil moisture sensor + sunlight sensor: m328p-sensors
- 2 Uno R3 m2560
    - OLED display shields for values on board near garden
    - Boards B & C (displays): m2560-oled-shields

# Board: Arduino Uno R3 m328p (sensors)

Written in Rust

Controls all sensors, captures soil moisture and sunlight inputs, batches them, and outputs them to the logger board. 

- Soil Moisture sensors:
    - [https://vetco.net/products/arduino-soil-moisture-sensor-d53](https://vetco.net/products/arduino-soil-moisture-sensor-d53)
- Sunlight sensors:
    - [https://www.robotshop.com/en/grove-sunlight-sensor.html](https://www.robotshop.com/en/grove-sunlight-sensor.html)

Connections:

- In: Soil & sunlight sensors
- Out-Channel 1: Analog out to m2560 OLED displays
- Out-Channel 2: SD/ethernet/analog data exfil (logger)
