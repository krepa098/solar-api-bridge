use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct PowerFlow {
    pub common: Common,
    pub inverters: Vec<PowerFlowInverter>,
    pub site: PowerFlowSite,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Common {
    pub datestamp: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PowerFlowInverter {
    #[serde(rename = "BatMode")]
    pub bat_mode: i32,
    #[serde(rename = "CID")]
    pub cid: i32,
    #[serde(rename = "DT")]
    pub dt: i32,
    #[serde(rename = "E_Total")]
    pub e_total: f32,
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "P")]
    pub p: f32,
    #[serde(rename = "SOC")]
    pub soc: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PowerFlowSite {
    #[serde(rename = "BatteryStandby")]
    pub battery_standby: bool,
    #[serde(rename = "E_Day")]
    pub e_day: Option<f32>,
    #[serde(rename = "E_Total")]
    pub e_total: f32,
    #[serde(rename = "E_Year")]
    pub e_year: Option<f32>,
    #[serde(rename = "MLoc")]
    pub mloc: i32,
    #[serde(rename = "Mode")]
    pub mode: String,
    #[serde(rename = "P_Akku")]
    pub p_akku: f32,
    #[serde(rename = "P_Grid")]
    pub p_grid: f32,
    #[serde(rename = "P_Load")]
    pub p_load: f32,
    #[serde(rename = "P_PV")]
    pub p_pv: f32,
    #[serde(rename = "rel_Autonomy")]
    pub rel_autonomy: f32,
    #[serde(rename = "rel_SelfConsumption")]
    pub rel_self_consumption: f32,
}

pub async fn request_power_flow(cfg: &Config) -> Result<PowerFlow> {
    const ENDPOINT: &str = "api/status/powerflow";
    let url = format!("{}/{}", cfg.inverter_url, ENDPOINT);
    let data = reqwest::get(url).await?.bytes().await?;
    let json = serde_json::from_slice(&data)?;

    Ok(json)
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDateTime, TimeZone};

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
    async fn test_power_flow() {
        let cfg = config();

        let data = request_power_flow(&cfg).await.unwrap();
        dbg!(&data);

        let datetime = NaiveDateTime::parse_from_str(
            &format!("{} {}", data.common.datestamp, data.common.timestamp),
            "%d.%m.%Y %H:%M:%S",
        )
        .unwrap();
        let utc = chrono::Local
            .from_local_datetime(&datetime)
            .unwrap()
            .to_utc();

        dbg!(utc);
    }
}

// 1983 Apr 13 12:09:14.274 +0000", "%Y %b %d %H:%M:%S%.3f %z
