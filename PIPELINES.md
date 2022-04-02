# Data Pipeline Schemas

All pipelines begin from soil & sunlight sensor data (sssd) and are interpreted by board ‘A’. (base line)

The base line is the constant relationship of board ‘A’ among other components in the pipeline.

### Def.

***Base** =* sssd → R3 m328p (interprets & formats sensor outputs)

***CU*** = Ubuntu machine 

### Pipeline Options

1. *Base* → ethernet → CU SQL Server
2. *Base* → USB? ethernet? → CU → Microsoft Excel → dump.csv → Python visualizer
3. *Base* → SD → CU → Excel → dump.csv
4. *Base →* analog out → analog receiver (connected to CU) → Ubuntu SQL Server