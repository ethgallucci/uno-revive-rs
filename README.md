# garden-microcontrollers-rs
monorepo for a garden monitoring project using Uno R3 m328p and m2560

## System Arch
The schema for this projects architecture can be found here:
[Project Schema](https://0xryuk.notion.site/Garden-1d7eddca07cc4968abf67541b919bdaa)
## Code
Each board runs unique code, repo is seperated by which board the code'll run on.
The m328p board is used for capturing input from sensors and logging it (writing data to an SD), and the 
m2560 board is simply used for displaying real-time data on OLED display shields so the monitoring station looks cool!
