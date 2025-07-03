# thermofeel-rs

A library to calculate human thermal comfort indexes.

This crate is a clone of the Python library [thermofeel](https://github.com/ecmwf/thermofeel). All credit for sourcing and implementing the algorithums used lies with the current maintainers:
- Claudia Di Napoli
- Tiago Quintino

and all else who contraubited towards the project.

Currently calculates the thermal indexes:
  - Universal Thermal Climate Index
  - Apparent Temperature
  - Heat Index Adjusted
  - Heat Index Simplified
  - Humidex
  - Normal Effective Temperature
  - Wet Bulb Globe Temperature
  - Wet Bulb Globe Temperature Simple
  - Wind Chill

In support of the above indexes, it also calculates:
  - Globe Temperature
  - Mean Radiant Temperature
  - Mean Radiant Temperature from Globe Temperature
  - Relative Humidity Percentage
  - Saturation vapour pressure
  - Wet Bulb Temperature

## Installation

Install with:
```
cargo add thermofeel-rs
```

