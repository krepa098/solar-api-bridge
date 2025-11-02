mod config;
mod db;
mod solar_api;
mod unstable_api;

use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, TimeZone};
use std::time::Duration;

use crate::{
    config::Config,
    db::{InverterReading, SmartMeterReading, StorageReading, UnstablePowerFlowReading, db_client},
    solar_api::{request_inverter_info, request_meter_info, request_storage_info},
    unstable_api::request_power_flow,
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
        let ch = &meter_info.body.data["0"];

        SmartMeterReading {
            time,
            power_real_p_sum: ch.power_real_p_sum,
            energy_real_wac_sum_consumed: ch.energy_real_wac_sum_consumed,
            energy_real_wac_sum_produced: ch.energy_real_wac_sum_produced,
            energy_real_wac_minus_absolute: ch.energy_real_wac_minus_absolute,
            energy_real_wac_plus_absolute: ch.energy_real_wac_plus_absolute,
            frequency_phase_average: ch.frequency_phase_average,
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

    let power_flow_reading = {
        let power_flow = request_power_flow(cfg).await?;
        let site = &power_flow.site;

        let datetime = NaiveDateTime::parse_from_str(
            &format!(
                "{} {}",
                power_flow.common.datestamp, power_flow.common.timestamp
            ),
            "%d.%m.%Y %H:%M:%S",
        )
        .unwrap();
        let time = chrono::Local
            .from_local_datetime(&datetime)
            .unwrap()
            .to_utc();

        UnstablePowerFlowReading {
            time,
            p_akku: site.p_akku,
            p_grid: site.p_grid,
            p_pv: site.p_pv,
            rel_autonomy: site.rel_autonomy,
            rel_self_consumption: site.rel_self_consumption,
        }
        .into_query("power_flow_reading")
    };

    db_client(cfg)
        .query(vec![
            smart_meter_reading,
            storage_reading,
            inverter_reading,
            power_flow_reading,
        ])
        .await?;

    Ok(())
}
