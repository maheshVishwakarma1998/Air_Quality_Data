#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::collections::HashMap;
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;
// ... (existing imports and types)

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct AirQualityData {
    id: u64,
    location: String,
    timestamp: u64,
    air_quality_index: u32,
    health_recommendations: String,
    pollutant_levels: HashMap<String, f64>,
    weather_conditions: WeatherData,
}

impl Storable for AirQualityData {
    // Implement Storable trait methods for serialization and deserialization
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for AirQualityData {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static AIR_QUALITY_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static AIR_QUALITY_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(AIR_QUALITY_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter for air quality data")
    );

    static AIR_QUALITY_STORAGE: RefCell<StableBTreeMap<u64, AirQualityData, Memory>> =
        RefCell::new(StableBTreeMap::init(
            AIR_QUALITY_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

// Helper method to perform insert for AirQualityData
fn do_insert_air_quality(data: &AirQualityData) {
    AIR_QUALITY_STORAGE.with(|service| service.borrow_mut().insert(data.id, data.clone()));
}

// Existing struct for weather conditions
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct WeatherData {
    temperature: f64,
    humidity: f64,
    wind_speed: f64,
}

// ... (existing thread-local variables and payload structure)

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct AirQualityUpdatePayload {
    location: String,
    air_quality_index: u32,
    health_recommendations: String,
    pollutant_levels: Option<HashMap<String, f64>>,
    weather_conditions: Option<WeatherData>,
}

// ... (existing functions)

// 2.7.8 get_air_quality_data Function:
#[ic_cdk::query]
fn get_air_quality_data(id: u64) -> Result<AirQualityData, Error> {
    match _get_air_quality_data(&id) {
        Some(data) => Ok(data),
        None => Err(Error::NotFound {
            msg: format!("air quality data with id={} not found", id),
        }),
    }
}

// 2.7.9 _get_air_quality_data Function:
fn _get_air_quality_data(id: &u64) -> Option<AirQualityData> {
    AIR_QUALITY_STORAGE.with(|s| s.borrow().get(id))
}

// 2.7.10 add_air_quality_data Function:
#[ic_cdk::update]
fn add_air_quality_data(data: AirQualityUpdatePayload) -> Option<AirQualityData> {
    let id = AIR_QUALITY_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter for air quality data");

    let pollutant_levels = data.pollutant_levels.unwrap_or_default();
    let weather_conditions = data.weather_conditions.unwrap_or_default();

    let air_quality_data = AirQualityData {
        id,
        location: data.location,
        timestamp: time(),
        air_quality_index: data.air_quality_index,
        health_recommendations: data.health_recommendations,
        pollutant_levels,
        weather_conditions,
    };

    do_insert_air_quality(&air_quality_data);
    Some(air_quality_data)
}

// 2.7.11 update_air_quality_data Function:
#[ic_cdk::update]
fn update_air_quality_data(
    id: u64,
    payload: AirQualityUpdatePayload,
) -> Result<AirQualityData, Error> {
    match AIR_QUALITY_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut data) => {
            data.location = payload.location;
            data.air_quality_index = payload.air_quality_index;
            data.health_recommendations = payload.health_recommendations;
            data.pollutant_levels = payload.pollutant_levels.unwrap_or_default();
            data.weather_conditions = payload.weather_conditions.unwrap_or_default();
            data.timestamp = time();

            do_insert_air_quality(&data);
            Ok(data)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update air quality data with id={}. data not found",
                id
            ),
        }),
    }
}

// 2.7.12 delete_air_quality_data Function:
#[ic_cdk::update]
fn delete_air_quality_data(id: u64) -> Result<AirQualityData, Error> {
    match AIR_QUALITY_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(data) => Ok(data),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete air quality data with id={}. data not found.",
                id
            ),
        }),
    }
}

#[ic_cdk::query]
fn get_all_air_quality_data() -> Result<Vec<AirQualityData>, Error> {
    Ok(AIR_QUALITY_STORAGE.with(|service| {
        let storage = service.borrow_mut();
        storage.iter().map(|(_, item)| item.clone()).collect()
    }))
}

#[ic_cdk::query]
fn search_air_quality_data_by_location(location: String) -> Result<Vec<AirQualityData>, Error> {
    Ok(AIR_QUALITY_STORAGE.with(|service| {
        let borrow = &*service.borrow();
        borrow
            .iter()
            .filter_map(|(_, space)| {
                if space.location.contains(&location) {
                    Some(space.clone())
                } else {
                    None
                }
            })
            .collect()
    }))
}

#[ic_cdk::query]
fn get_air_quality_data_by_weather_conditions(
    min_temperature: f64,
    max_temperature: f64,
    min_humidity: f64,
    max_humidity: f64,
    min_wind_speed: f64,
    max_wind_speed: f64,
) -> Result<Vec<AirQualityData>, Error> {
    Ok(AIR_QUALITY_STORAGE.with(|service| {
        let borrow = service.borrow();
        borrow
            .iter()
            .filter_map(|(_, data)| {
                let weather = &data.weather_conditions;
                if weather.temperature >= min_temperature
                    && weather.temperature <= max_temperature
                    && weather.humidity >= min_humidity
                    && weather.humidity <= max_humidity
                    && weather.wind_speed >= min_wind_speed
                    && weather.wind_speed <= max_wind_speed
                {
                    Some(data.clone())
                } else {
                    None
                }
            })
            .collect()
    }))
}

#[ic_cdk::query]
fn get_air_quality_data_by_pollutant_level(
    pollutant: String,
    min_level: f64,
    max_level: f64,
) -> Result<Vec<AirQualityData>, Error> {
    Ok(AIR_QUALITY_STORAGE.with(|service| {
        let borrow = service.borrow();
        borrow
            .iter()
            .filter_map(|(_, data)| {
                if let Some(level) = data.pollutant_levels.get(&pollutant) {
                    if *level >= min_level && *level <= max_level {
                        Some(data.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }))
}

#[ic_cdk::query]
fn get_air_quality_data_by_timestamp_range(
    start_timestamp: u64,
    end_timestamp: u64,
) -> Result<Vec<AirQualityData>, Error> {
    Ok(AIR_QUALITY_STORAGE.with(|service| {
        let borrow = service.borrow();
        borrow
            .iter()
            .filter_map(|(_, data)| {
                if data.timestamp >= start_timestamp && data.timestamp <= end_timestamp {
                    Some(data.clone())
                } else {
                    None
                }
            })
            .collect()
    }))
}

// Enum for error handling
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

// Export Candid interface definitions for the canister
ic_cdk::export_candid!();
