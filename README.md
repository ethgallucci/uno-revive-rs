
<h1 align="left">
  <p align="left">
      uno-revive-rs
  <img alt="GitHub release (latest SemVer including pre-releases)" src="https://img.shields.io/github/v/release/ethgallucci/uno-revive-rs?include_prereleases&style=plastic">
  <a href="https://github.com/ethgallucci/uno-revive-rs/graphs/commit-activity">
      <img alt="Commit activity" src="https://img.shields.io/github/commit-activity/m/ethgallucci/uno-revive-rs?style=plastic" />
  </a>
  <img alt="Lines of code" src="https://img.shields.io/tokei/lines/github/ethgallucci/uno-revive-rs?color=olive&style=plastic">
  <img alt="GitHub issues" src="https://img.shields.io/github/issues/ethgallucci/uno-revive-rs?color=white&style=plastic">
</p>
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

Controls all sensors, captures soil moisture and sunlight inputs, batches them, and outputs them to the logger board. 

- Soil Moisture sensors:
    - [https://vetco.net/products/arduino-soil-moisture-sensor-d53](https://vetco.net/products/arduino-soil-moisture-sensor-d53)
- Sunlight sensors:
    - [https://www.robotshop.com/en/grove-sunlight-sensor.html](https://www.robotshop.com/en/grove-sunlight-sensor.html)

Connections:

- In: Soil & sunlight sensors
- Out-Channel 1: m2560 OLED displays
- Out-Channel 2: SD (logger)

# Dev
There are a few ways to compile the system.

  With both boards plugged in (builds and flashes to board)
  ```sh
  avr-garduino/m328-sensors $ cargo run
  avr-garduino/m2560-oled-shields $ cargo run
  ```

  From root with provided aliases:
  ```sh
  avr-garduino $ cargo build-328p && cargo build-2560
  ```
  From root manual:
  ```sh
  avr-garduino $ cargo build --verbose --package m328p-sensors --target=m328p-sensors/spec/avr-atmega328p.json
  avr-garduino $ cargo build --verbose --package m2560-oled-shields --target=m2560-oled-shields/spec/avr-atmega2560.json
  ```
  From package:
  ```sh
  avr-garduino/m328p-sensors $ cargo build --verbose
  avr-garduino/m2560-oled-shields $ cargo build --verbose
  ```
# Special Thanks
I want to extend my gratitude to the contributors of the [avr-hal](https://github.com/rahix/avr-hal) crate.
This project couldn't have done it without them :)


[commit-shield]: https://img.shields.io/github/commit-activity/w/ethgallucci/uno-revive-rs?style=plastic
[commit-url]: https://github.com/ethgallucci/uno-revive-rs/commits/main
