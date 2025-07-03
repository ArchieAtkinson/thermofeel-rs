# thermofeel-rs

A library to calculate human thermal comfort indexes.

This crate is a (almost) clone of the Python library [thermofeel](https://github.com/ecmwf/thermofeel).All credit for sourcing and implementing the algorithums used lies with the current maintainers:
- Claudia Di Napoli
- Tiago Quintino
- and anyone else who contributed towards the project

This crate implements commit [`62f754b`](https://github.com/ecmwf/thermofeel/commit/62f754b7fe89fefc0789b68f9b96e58952386377).

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

## Differences from the Python Lib

The only significant change is the use single values instead of arrays for argument and return types. The was primarily for simplicity in porting and my current needs for the project. However, I would be open suggestion of alternative implementations that may be more suitable for large datasets.  
