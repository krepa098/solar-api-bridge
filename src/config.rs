pub struct Config {
    pub inverter_url: String,
    pub influxdb_url: String,
    pub influxdb_database: String,
    pub influxdb_user: String,
    pub influxdb_password: String,
    pub interval_s: u64,
}

impl Config {
    pub fn new_from_env() -> Self {
        Self {
            inverter_url: std::env::var("INVERTER_URL").unwrap_or("http://localhost".to_string()),
            influxdb_url: std::env::var("INFLUXDB_URL").unwrap_or("http://localhost".to_string()),
            influxdb_database: std::env::var("INFLUXDB_DATABASE").unwrap_or("solar".to_string()),
            influxdb_user: std::env::var("INFLUXDB_USER").unwrap_or("admin".to_string()),
            influxdb_password: std::env::var("INFLUXDB_PASSWORD").unwrap_or("admin".to_string()),
            interval_s: std::env::var("INTERVAL")
                .map(|interval| interval.parse().unwrap_or(60))
                .unwrap_or(60),
        }
    }

    pub fn print(&self) {
        println!("config:");
        println!("- inverter url: {}", self.inverter_url);
        println!("- influxdb url: {}", self.influxdb_url);
        println!("- influxdb database: {}", self.influxdb_database);
        println!("- influxdb user: {}", self.influxdb_user);
        println!("- interval: {}s", self.interval_s);
    }
}
