use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use rand::Rng; // Rastgele sayı üretimi için
use log::{info, warn, error}; // Loglama için
use serde::{Serialize, Deserialize}; // Serileştirme için
use std::fs::File; // Dosya işlemleri için
use std::io::{self, Write, BufReader, BufRead}; // Giriş/Çıkış işlemleri için

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    logging_enabled: bool,
    anomaly_threshold: u64,
    monitoring_interval: u64,
    max_packet_count: u64,
    error_threshold: u64,
}

#[derive(Debug, Clone)]
pub struct Monitor {
    logging_enabled: bool,
    traffic_data: Arc<Mutex<HashMap<String, u64>>>,
    anomaly_threshold: u64,
    monitoring_interval: u64,
    error_threshold: u64,
    start_time: Instant,
}

impl Monitor {
    pub fn new(config: Config) -> Self {
        Self {
            logging_enabled: config.logging_enabled,
            traffic_data: Arc::new(Mutex::new(HashMap::new())),
            anomaly_threshold: config.anomaly_threshold,
            monitoring_interval: config.monitoring_interval,
            error_threshold: config.error_threshold,
            start_time: Instant::now(),
        }
    }

    pub fn start(&self) {
        println!("Real-time monitoring started...");
        if self.logging_enabled {
            self.log_activity();
        }
        self.monitor_traffic();
    }

    fn log_activity(&self) {
        println!("Logging network activity...");
        env_logger::init(); // Loglama için
        info!("Network activity logging enabled.");
    }

    fn monitor_traffic(&self) {
        let traffic_data = Arc::clone(&self.traffic_data);
        let monitoring_interval = self.monitoring_interval;

        thread::spawn(move || loop {
            let mut data = traffic_data.lock().unwrap();
            // Simüle edilmiş ağ trafiği verilerini güncelleyin
            let random_packet_count: u64 = rand::thread_rng().gen_range(50..500);
            data.insert("total_packets".to_string(), random_packet_count);
            data.insert("timestamp".to_string(), Instant::now().elapsed().as_secs());
            data.insert("error_count".to_string(), 0); // Hata sayısını sıfırla
            println!("Monitored traffic: {} packets", random_packet_count);

            // Verileri analiz et
            if let Some(monitor) = MONITOR.lock().unwrap().as_ref() {
                monitor.analyze_data();
            }

            thread::sleep(Duration::from_secs(monitoring_interval));
        });
    }

    pub fn analyze_data(&self) {
        let data = self.traffic_data.lock().unwrap();
        println!("Analyzing data: {:?}", *data);

        if let Some(total_packets) = data.get("total_packets") {
            if *total_packets > self.anomaly_threshold {
                warn!("Anomaly detected! High packet count: {}", total_packets);
                // Anomali yanıt mantığı burada
            } else {
                info!("No anomalies detected. Packet count is within normal range.");
            }
        } else {
            error!("Total packets data not found!");
        }

        // Ek analiz yap
        self.analyze_traffic_patterns(&data);
        self.check_for_errors(&data);
    }

    fn analyze_traffic_patterns(&self, data: &HashMap<String, u64>) {
        if let Some(timestamp) = data.get("timestamp") {
            let current_time = Instant::now().elapsed().as_secs();
            let duration = current_time - timestamp;

            // Süreye göre trafik desenlerini analiz et
            if duration > 60 {
                warn!("Monitoring has been running for more than 60 seconds!");
            }
        }
    }

    fn check_for_errors(&self, data: &HashMap<String, u64>) {
        if let Some(error_count) = data.get("error_count") {
            if *error_count > self.error_threshold {
                warn!("Error threshold exceeded! Error count: {}", error_count);
                // Hata yanıt mantığı burada
            } else {
                info!("Error count is within normal range.");
            }
        } else {
            error!("Error count data not found!");
        }
    }

    fn save_traffic_data(&self) {
        let data = self.traffic_data.lock().unwrap();
        let file = File::create("traffic_log.txt").expect("Unable to create log file");
        let mut writer = io::BufWriter::new(file);
        
        writeln!(writer, "Traffic Log:").expect("Unable to write data");
        for (key, value) in data.iter() {
            writeln!(writer, "{}: {}", key, value).expect("Unable to write data");
        }
        info!("Traffic data saved to traffic_log.txt");
    }

    fn load_traffic_data(&self) {
        let file = File::open("traffic_log.txt").expect("Unable to open log file");
        let reader = BufReader::new(file);
        
        for line in reader.lines() {
            let line = line.expect("Unable to read line");
            println!("Log: {}", line);
        }
    }

    fn reset_traffic_data(&self) {
        let mut data = self.traffic_data.lock().unwrap();
        data.clear();
        info!("Traffic data reset.");
    }
}

// Global monitor instance for data analysis
lazy_static::lazy_static! {
    static ref MONITOR: Mutex<Option<Monitor>> = Mutex::new(None);
}

// Fonksiyon: yapılandırmayı yükle
fn load_config() -> Config {
    // Örnek yapılandırma dosyası oluştur
    Config {
        logging_enabled: true,
        anomaly_threshold: 400,
        monitoring_interval: 5,
        max_packet_count: 1000,
        error_threshold: 10,
    }
}

// Ana fonksiyon
fn main() {
    let config = load_config();
    let monitor = Monitor::new(config.clone()); // Loglamayı etkinleştir
    *MONITOR.lock().unwrap() = Some(monitor.clone()); // Analiz erişimi için izleyiciyi sakla
    monitor.start();

    // Veri kaydetme, yükleme ve sıfırlama işlevselliği
    loop {
        thread::sleep(Duration::from_secs(60));
        
        // Belirli bir süre sonra verileri kaydet
        if Instant::now().duration_since(monitor.start_time).as_secs() > 300 {
            monitor.save_traffic_data();
            monitor.load_traffic_data();
            monitor.reset_traffic_data();
        }
    }
}
