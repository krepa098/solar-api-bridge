mod config;
mod db;
mod solar_api;

use anyhow::Result;
use chrono::DateTime;
use std::time::Duration;

use crate::{
    config::Config,
    db::{InverterReading, SmartMeterReading, StorageReading, db_client},
    solar_api::{request_inverter_info, request_meter_info, request_storage_info},
};
use influxdb::InfluxDbWriteable;

#[tokio::main]
async fn main() {
    println!("{} {}", env!("CARGO_CRATE_NAME"), env!("CARGO_PKG_VERSION"));
    let cfg = Config::new_from_env();
    cfg.print();

    loop {
        match poll_and_push(&cfg).await {
            Ok(_) => println!("Push db"),
            Err(e) => println!("err: {}", e),
        };

        tokio::time::sleep(Duration::from_secs(cfg.interval_s)).await;
    }
}

async fn poll_and_push(cfg: &Config) -> Result<()> {
    let smart_meter_reading = {
        let meter_info = request_meter_info(cfg).await?;
        let time = DateTime::from_timestamp_secs(meter_info.body.data["0"].time_stamp as i64)
            .ok_or(anyhow::anyhow!("malformed timestamp"))?;
        let channel = &meter_info.body.data["0"];

        SmartMeterReading {
            time,
            power_real_p_sum: channel.power_real_p_sum,
            energy_real_wac_sum_consumed: channel.energy_real_wac_sum_consumed,
            energy_real_wac_sum_produced: channel.energy_real_wac_sum_produced,
            energy_real_wac_minus_absolute: channel.energy_real_wac_minus_absolute,
            energy_real_wac_plus_absolute: channel.energy_real_wac_plus_absolute,
        }
        .into_query("smart_meter_reading")
    };

    let storage_reading = {
        let storage_info = request_storage_info(cfg).await?;
        let controller = &storage_info.body.data["0"].controller;
        let time = DateTime::from_timestamp_secs(controller.time_stamp as i64)
            .ok_or(anyhow::anyhow!("malformed timestamp"))?;

        StorageReading {
            time,
            state_of_charge_relative: controller.state_of_charge_relative,
            temperature_cell: controller.temperature_cell,
            voltage_dc: controller.voltage_dc,
            current_dc: controller.current_dc,
        }
        .into_query("storage_reading")
    };

    let inverter_reading = {
        let inverter_info = request_inverter_info(cfg).await?;
        let data = &inverter_info.body.data;
        let time = DateTime::parse_from_rfc3339(&inverter_info.head.time_stamp)?.to_utc();

        InverterReading {
            time,
            pac: data.pac.values["1"].ok_or(anyhow::anyhow!("missing pac value"))?,
        }
        .into_query("inverter_reading")
    };

    db_client(cfg)
        .query(vec![smart_meter_reading, storage_reading, inverter_reading])
        .await?;

    Ok(())
}
