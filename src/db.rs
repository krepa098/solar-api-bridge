use chrono::{DateTime, Utc};
use influxdb::{Client, InfluxDbWriteable};

use crate::config::Config;

#[derive(InfluxDbWriteable)]
pub struct StorageReading {
    pub time: DateTime<Utc>,
    pub state_of_charge_relative: f32,
    pub temperature_cell: f32,
    pub voltage_dc: f32,
    pub current_dc: f32,
}

#[derive(InfluxDbWriteable)]
pub struct SmartMeterReading {
    pub time: DateTime<Utc>,
    pub power_real_p_sum: f32,
    pub energy_real_wac_sum_produced: f32,
    pub energy_real_wac_sum_consumed: f32,
    pub energy_real_wac_plus_absolute: f32,
    pub energy_real_wac_minus_absolute: f32,
    pub frequency_phase_average: f32,
}

#[derive(InfluxDbWriteable)]
pub struct InverterReading {
    pub time: DateTime<Utc>,
    pub pac: f32,
}

#[derive(InfluxDbWriteable)]
pub struct UnstablePowerFlowReading {
    pub time: DateTime<Utc>,
    pub rel_autonomy: f32,
    pub rel_self_consumption: f32,
    pub p_pv: f32,
    pub p_grid: f32,
    pub p_akku: f32,
}

pub fn db_client(cfg: &Config) -> Client {
    influxdb::Client::new(&cfg.influxdb_url, &cfg.influxdb_database)
        .with_auth(&cfg.influxdb_user, &cfg.influxdb_password)
}
