use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::collections::HashMap;

use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Head {
    #[serde(rename = "RequestArguments")]
    pub request_arguments: Map<String, serde_json::Value>,
    #[serde(rename = "Status")]
    pub status: Map<String, serde_json::Value>,
    #[serde(rename = "Timestamp")]
    pub time_stamp: String, // RFC3339
}

/**************************
    MeterRealtimeData
**************************/
#[derive(Serialize, Deserialize, Debug)]
pub struct MeterRealtimeData {
    #[serde(rename = "Head")]
    pub head: Head,
    #[serde(rename = "Body")]
    pub body: MeterRealtimeDataBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeterRealtimeDataBody {
    #[serde(rename = "Data")]
    pub data: HashMap<String, MeterRealtimeDataChannel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeterRealtimeDataChannel {
    #[serde(rename = "Current_AC_Phase_1")]
    pub current_ac_phase_1: f32,
    #[serde(rename = "Current_AC_Phase_2")]
    pub current_ac_phase_2: f32,
    #[serde(rename = "Current_AC_Phase_3")]
    pub current_ac_phase_3: f32,
    #[serde(rename = "Current_AC_Sum")]
    pub current_ac_sum: f32,
    #[serde(rename = "Details")]
    pub details: Map<String, serde_json::Value>,
    #[serde(rename = "Enable")]
    pub enable: i32,
    #[serde(rename = "EnergyReal_WAC_Minus_Absolute")]
    pub energy_real_wac_minus_absolute: f32,
    #[serde(rename = "EnergyReal_WAC_Phase_1_Consumed")]
    pub energy_real_wac_phase_1_consumed: f32,
    #[serde(rename = "EnergyReal_WAC_Phase_1_Produced")]
    pub energy_real_wac_phase_1_produced: f32,
    #[serde(rename = "EnergyReal_WAC_Phase_2_Consumed")]
    pub energy_real_wac_phase_2_consumed: f32,
    #[serde(rename = "EnergyReal_WAC_Phase_2_Produced")]
    pub energy_real_wac_phase_2_produced: f32,
    #[serde(rename = "EnergyReal_WAC_Phase_3_Consumed")]
    pub energy_real_wac_phase_3_consumed: f32,
    #[serde(rename = "EnergyReal_WAC_Phase_3_Produced")]
    pub energy_real_wac_phase_3_produced: f32,
    #[serde(rename = "EnergyReal_WAC_Plus_Absolute")]
    pub energy_real_wac_plus_absolute: f32,
    #[serde(rename = "EnergyReal_WAC_Sum_Consumed")]
    pub energy_real_wac_sum_consumed: f32,
    #[serde(rename = "EnergyReal_WAC_Sum_Produced")]
    pub energy_real_wac_sum_produced: f32,
    #[serde(rename = "Frequency_Phase_Average")]
    pub frequency_phase_average: f32,
    #[serde(rename = "Meter_Location_Current")]
    pub meter_location_current: f32,
    #[serde(rename = "PowerApparent_S_Phase_1")]
    pub power_apparent_s_phase_1: f32,
    #[serde(rename = "PowerApparent_S_Phase_2")]
    pub power_apparent_s_phase_2: f32,
    #[serde(rename = "PowerApparent_S_Phase_3")]
    pub power_apparent_s_phase_3: f32,
    #[serde(rename = "PowerApparent_S_Sum")]
    pub power_apparent_s_sum: f32,
    #[serde(rename = "PowerFactor_Phase_1")]
    pub power_factor_phase_1: f32,
    #[serde(rename = "PowerFactor_Phase_2")]
    pub power_factor_phase_2: f32,
    #[serde(rename = "PowerFactor_Phase_3")]
    pub power_factor_phase_3: f32,
    #[serde(rename = "PowerFactor_Sum")]
    pub power_factor_sum: f32,
    #[serde(rename = "PowerReactive_Q_Phase_1")]
    pub power_reactive_q_phase_1: f32,
    #[serde(rename = "PowerReactive_Q_Phase_2")]
    pub power_reactive_q_phase_2: f32,
    #[serde(rename = "PowerReactive_Q_Phase_3")]
    pub power_reactive_q_phase_3: f32,
    #[serde(rename = "PowerReactive_Q_Sum")]
    pub power_reactive_q_sum: f32,
    #[serde(rename = "PowerReal_P_Phase_1")]
    pub power_real_p_phase_1: f32,
    #[serde(rename = "PowerReal_P_Phase_2")]
    pub power_real_p_phase_2: f32,
    #[serde(rename = "PowerReal_P_Phase_3")]
    pub power_real_p_phase_3: f32,
    #[serde(rename = "PowerReal_P_Sum")]
    pub power_real_p_sum: f32,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: u64,
    #[serde(rename = "Visible")]
    pub visible: i32,
    #[serde(rename = "Voltage_AC_PhaseToPhase_12")]
    pub voltage_ac_phase_to_phase_12: f32,
    #[serde(rename = "Voltage_AC_PhaseToPhase_23")]
    pub voltage_ac_phase_to_phase_23: f32,
    #[serde(rename = "Voltage_AC_PhaseToPhase_31")]
    pub voltage_ac_phase_to_phase_31: f32,
    #[serde(rename = "Voltage_AC_Phase_1")]
    pub voltage_ac_phase_1: f32,
    #[serde(rename = "Voltage_AC_Phase_2")]
    pub voltage_ac_phase_2: f32,
    #[serde(rename = "Voltage_AC_Phase_3")]
    pub voltage_ac_phase_3: f32,
    #[serde(rename = "Voltage_AC_Phase_Average")]
    pub voltage_ac_phase_average: f32,
}

/**************************
    StorageRealtimeData
**************************/
#[derive(Serialize, Deserialize, Debug)]
pub struct StorageRealtimeData {
    #[serde(rename = "Head")]
    pub head: Head,
    #[serde(rename = "Body")]
    pub body: StorageRealtimeDataBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageRealtimeDataBody {
    #[serde(rename = "Data")]
    pub data: HashMap<String, StorageRealtimeDataChannel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageRealtimeDataChannel {
    #[serde(rename = "Controller")]
    pub controller: StorageRealtimeDataController,
    #[serde(rename = "Modules")]
    #[serde(skip)]
    pub modules: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageRealtimeDataController {
    #[serde(rename = "Capacity_Maximum")]
    pub capacity_maximum: f32, // current max capacity
    #[serde(rename = "Current_DC")]
    pub current_dc: f32, // battery output current (+ charging)
    #[serde(rename = "DesignedCapacity")]
    pub designed_capacity: f32, // max designed capacity
    #[serde(rename = "Details")]
    pub details: HashMap<String, String>,
    #[serde(rename = "Enable")]
    pub enable: i32, // device is managed (1.0) or disconnected (0.0)
    #[serde(rename = "StateOfCharge_Relative")]
    pub state_of_charge_relative: f32, // relative charged capacity in %
    #[serde(rename = "Status_BatteryCell")]
    pub status_battery_cell: f32,
    #[serde(rename = "Temperature_Cell")]
    pub temperature_cell: f32, // temperature in degree celsius
    #[serde(rename = "TimeStamp")]
    pub time_stamp: u64, // last timestamp data has been refrehsed
    #[serde(rename = "Voltage_DC")]
    pub voltage_dc: f32, // battery output voltage
}

/**************************
    InverterRealtimeData
**************************/
#[derive(Serialize, Deserialize, Debug)]
pub struct InverterRealtimeData {
    #[serde(rename = "Head")]
    pub head: Head,
    #[serde(rename = "Body")]
    pub body: InverterRealtimeDataBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InverterRealtimeDataBody {
    #[serde(rename = "Data")]
    pub data: InverterRealtimeDataChannel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InverterRealtimeDataChannel {
    #[serde(rename = "DAY_ENERGY")]
    pub day_energy: Quantity, // GEN24/Tauro/Verto: will always report null
    #[serde(rename = "PAC")]
    pub pac: Quantity, // AC Power
    #[serde(rename = "TOTAL_ENERGY")]
    pub total_energy: Quantity, // AC Energy generated overall, updated ev­ery 5min
    #[serde(rename = "YEAR_ENERGY")]
    pub year_energy: Quantity, // GEN24/Tauro/Verto: will always report null
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quantity {
    #[serde(rename = "Unit")]
    pub unit: String,
    #[serde(rename = "Values")]
    pub values: HashMap<String, Option<f32>>,
}

/**************************
    Helpers
**************************/

pub async fn request_meter_info(cfg: &Config) -> Result<MeterRealtimeData> {
    const ENDPOINT: &str = "solar_api/v1/GetMeterRealtimeData.cgi";
    let url = format!("{}/{}", cfg.inverter_url, ENDPOINT);
    let data = reqwest::get(url).await?.bytes().await?;
    let json = serde_json::from_slice(&data)?;

    Ok(json)
}

pub async fn request_storage_info(cfg: &Config) -> Result<StorageRealtimeData> {
    const ENDPOINT: &str = "solar_api/v1/GetStorageRealtimeData.cgi";
    let url = format!("{}/{}", cfg.inverter_url, ENDPOINT);
    let data = reqwest::get(url).await?.bytes().await?;
    let json = serde_json::from_slice(&data)?;

    Ok(json)
}

pub async fn request_inverter_info(cfg: &Config) -> Result<InverterRealtimeData> {
    const ENDPOINT: &str = "solar_api/v1/GetInverterRealtimeData.cgi";
    let url = format!("{}/{}", cfg.inverter_url, ENDPOINT);
    let data = reqwest::get(url).await?.bytes().await?;
    let json = serde_json::from_slice(&data)?;

    Ok(json)
}

#[cfg(test)]
mod test {
    use super::*;

    fn config() -> Config {
        Config {
            inverter_url: "http://fronius".to_string(),
            influxdb_url: "".to_string(),
            influxdb_database: "".to_string(),
            influxdb_user: "".to_string(),
            influxdb_password: "".to_string(),
            interval_s: 0,
        }
    }

    #[tokio::test]
    async fn test_smart_meter() {
        let cfg = config();

        let data = request_meter_info(&cfg).await.unwrap();
        dbg!(data);
    }

    #[tokio::test]
    async fn test_inverter() {
        let cfg = config();

        let data = request_inverter_info(&cfg).await.unwrap();
        dbg!(data);
    }

    #[tokio::test]
    async fn test_storage() {
        let cfg = config();

        let data = request_storage_info(&cfg).await.unwrap();
        dbg!(data);
    }
}
