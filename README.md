# Air Quality Data Service

This repository contains a Canister for managing air quality data. The service provides functionalities to add, delete, update, and retrieve information about air quality entries. It includes features like querying data by pollutant level, timestamp range, weather conditions, and location.

## Data Structures

### `AirQualityData`
A struct representing air quality data with attributes such as ID, pollutant levels, air quality index, weather conditions, timestamp, location, and health recommendations.

### `AirQualityUpdatePayload`
A payload structure for updating air quality data, including pollutant levels, air quality index, weather conditions, location, and health recommendations.

### `Error`
Represents error types, including a `NotFound` variant with a descriptive message.

### `Result`
A variant representing the result of operations. Includes an `Ok` variant with `AirQualityData` or `Result_1` (a vector of `AirQualityData`), or an `Err` variant with an `Error`.

### `WeatherData`
A struct representing weather conditions with attributes such as wind speed, temperature, and humidity.

## Service Functions

1. **add_air_quality_data:**
   - Adds air quality data based on the provided `AirQualityUpdatePayload`.

2. **delete_air_quality_data:**
   - Deletes air quality data by ID.

3. **get_air_quality_data:**
   - Retrieves detailed information about air quality data by ID.

4. **get_air_quality_data_by_pollutant_level:**
   - Retrieves air quality data based on pollutant levels.

5. **get_air_quality_data_by_timestamp_range:**
   - Retrieves air quality data within a specified timestamp range.

6. **get_air_quality_data_by_weather_conditions:**
   - Retrieves air quality data based on weather conditions.

7. **get_all_air_quality_data:**
   - Retrieves all air quality data.

8. **search_air_quality_data_by_location:**
   - Searches air quality data by location.

9. **update_air_quality_data:**
   - Updates air quality data by ID using the provided `AirQualityUpdatePayload`.

## Candid Interface

The canister exports its Candid interface definitions using the `ic_cdk::export_candid!()` macro.

## Error Handling

Errors are represented using the `Error` enum, which includes a `NotFound` variant with a descriptive message.

Feel free to explore and integrate this canister into your Internet Computer project for efficient air quality data management!