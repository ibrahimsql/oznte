use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

pub struct ServiceStats {
    uptime: Duration,
    errors: usize,
}

pub struct Service {
    is_active: bool,
    stats: ServiceStats,
    bssid: String,
}

pub struct FakeNetwork {
    subnet: String,
    services: HashMap<String, Service>, // Hizmet adı ve hizmet bilgileri
    active_services: Vec<String>,
    event_log: Vec<String>,
}

impl FakeNetwork {
    // Yeni bir FakeNetwork örneği oluşturma
    pub fn new(subnet: &str) -> Self {
        let mut services = HashMap::new();
        services.insert("http".to_string(), Service {
            is_active: true,
            stats: ServiceStats { uptime: Duration::new(0, 0), errors: 0 },
            bssid: "00:11:22:33:44:55".to_string(),
        });
        services.insert("ssh".to_string(), Service {
            is_active: true,
            stats: ServiceStats { uptime: Duration::new(0, 0), errors: 0 },
            bssid: "00:11:22:33:44:66".to_string(),
        });
        services.insert("dns".to_string(), Service {
            is_active: false,
            stats: ServiceStats { uptime: Duration::new(0, 0), errors: 0 },
            bssid: "00:11:22:33:44:77".to_string(),
        });
        
        Self {
            subnet: subnet.to_string(),
            services,
            active_services: Vec::new(),
            event_log: Vec::new(),
        }
    }

    // Ağdaki aktif hizmetleri dağıtma
    pub fn deploy(&mut self) {
        println!("Subnet {} üzerinde sahte ağ dağıtılıyor...", self.subnet);
        for (service_name, service) in &self.services {
            if service.is_active {
                println!("{} hizmeti (BSSID: {}) başlatılıyor...", service_name, service.bssid);
                self.active_services.push(service_name.clone());
                self.log_event(format!("{} hizmeti (BSSID: {}) başlatıldı.", service_name, service.bssid));
                let service_name_clone = service_name.clone(); // Hizmet adını thread'e taşı
                thread::spawn(move || {
                    // Simüle edilmiş hizmet çalıştırma
                    let start_time = Instant::now();
                    loop {
                        println!("{} hizmeti (BSSID: {}) çalışıyor...", service_name_clone, service.bssid);
                        thread::sleep(Duration::from_secs(10));
                        // Her döngü iterasyonunda uptime'ı güncelle
                    }
                });
            }
        }
    }

    // Aktif hizmetleri izleme
    pub fn monitor_services(&mut self) {
        for service_name in &self.active_services {
            println!("{} hizmeti izleniyor...", service_name);
            if let Some(service) = self.services.get_mut(service_name) {
                if service.is_active {
                    service.stats.uptime += Duration::from_secs(10); // Simüle edilmiş zaman dilimi
                    // Hata oluşumunu simüle et
                    if rand::random::<f32>() < 0.1 { // %10 hata olasılığı
                        service.stats.errors += 1;
                        println!("{} hizmetinde (BSSID: {}) hata oluştu! Toplam hata: {}", service_name, service.bssid, service.stats.errors);
                    }
                }
                // Hatalar eşiği aşıldığında hizmeti yeniden başlat
                if service.stats.errors > 3 {
                    self.restart_service(service_name);
                }
            }
        }
    }

    // Belirli bir hizmeti durdurma
    pub fn stop_service(&mut self, service_name: &str) {
        if let Some(service) = self.services.get_mut(service_name) {
            service.stats.uptime = Duration::new(0, 0); // Uptime'ı sıfırla
            println!("{} hizmeti (BSSID: {}) durduruldu.", service_name, service.bssid);
            self.log_event(format!("{} hizmeti (BSSID: {}) durduruldu.", service_name, service.bssid));
        } else {
            println!("Hizmet {} bulunamadı.", service_name);
        }
    }

    // Yeni bir hizmet ekleme
    pub fn add_service(&mut self, service_name: &str, is_active: bool, bssid: &str) {
        self.services.insert(service_name.to_string(), Service {
            is_active,
            stats: ServiceStats { uptime: Duration::new(0, 0), errors: 0 },
            bssid: bssid.to_string(),
        });
        if is_active {
            self.active_services.push(service_name.to_string());
            self.log_event(format!("{} hizmeti (BSSID: {}) eklendi.", service_name, bssid));
        }
        println!("{} hizmeti (BSSID: {}) eklendi.", service_name, bssid);
    }

    // Mevcut bir hizmeti kaldırma
    pub fn remove_service(&mut self, service_name: &str) {
        if let Some(service) = self.services.remove(service_name) {
            self.active_services.retain(|s| s != service_name);
            println!("{} hizmeti (BSSID: {}) kaldırıldı.", service_name, service.bssid);
            self.log_event(format!("{} hizmeti (BSSID: {}) kaldırıldı.", service_name, service.bssid));
        } else {
            println!("Hizmet {} bulunamadı.", service_name);
        }
    }

    // Belirli bir hizmetin durumunu kontrol etme
    pub fn check_service_status(&self, service_name: &str) {
        if let Some(service) = self.services.get(service_name) {
            println!("{} hizmeti (BSSID: {}) şu anda {}", service_name, service.bssid, if service.is_active { "aktif" } else { "pasif" });
            println!("Uptime: {:?}", service.stats.uptime);
            println!("Hatalar: {}", service.stats.errors);
        } else {
            println!("Hizmet {} mevcut değil.", service_name);
        }
    }

    // Belirli bir hizmeti yeniden başlatma
    pub fn restart_service(&mut self, service_name: &str) {
        if let Some(service) = self.services.get_mut(service_name) {
            println!("{} hizmeti (BSSID: {}) yeniden başlatılıyor...", service_name, service.bssid);
            service.stats.uptime = Duration::new(0, 0); // Uptime'ı sıfırla
            service.stats.errors = 0; // Hataları sıfırla
            self.log_event(format!("{} hizmeti (BSSID: {}) yeniden başlatıldı.", service_name, service.bssid));
            // İsteğe bağlı olarak hizmeti yeniden dağıtabilirsin
        } else {
            println!("Hizmet {} yeniden başlatmak için bulunamadı.", service_name);
        }
    }

    // Tüm hizmetlerin durumunu kaydetme
    pub fn log_service_status(&self) {
        for (service_name, service) in &self.services {
            println!("{} hizmetinin durumu kaydediliyor...", service_name);
            println!("Uptime: {:?}", service.stats.uptime);
            println!("Hatalar: {}", service.stats.errors);
            println!("BSSID: {}", service.bssid);
        }
    }

    // Olayları bir dosyaya kaydetme
    fn log_event(&mut self, event: String) {
        self.event_log.push(event.clone());
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("event_log.txt")
            .expect("Log dosyası açılamadı");
        writeln!(file, "{}", event).expect("Log dosyasına yazılamadı");
    }

    // Olay kaydını yazdırma
    pub fn print_event_log(&self) {
        println!("Olay Kaydı:");
        for event in &self.event_log {
            println!("{}", event);
        }
    }

    // Tüm hizmetlerin istatistiklerini alma
    pub fn retrieve_service_statistics(&self) {
        println!("Hizmet İstatistikleri:");
        for (service_name, service) in &self.services {
            println!("Hizmet: {}", service_name);
            println!(" - Uptime: {:?}", service.stats.uptime);
            println!(" - Hatalar: {}", service.stats.errors);
            println!(" - BSSID: {}", service.bssid);
        }
    }

    // Hizmet parametrelerini yapılandırma
    pub fn configure_service(&mut self, service_name: &str, is_active: bool) {
        if let Some(service) = self.services.get_mut(service_name) {
            service.stats.errors = 0; // Yapılandırma değiştiğinde hataları sıfırla
            println!("{} hizmeti (BSSID: {}) {} olarak yapılandırılıyor.", service_name, service.bssid, if is_active { "aktif" } else { "pasif" });
            service.is_active = is_active;
            self.log_event(format!("{} hizmeti (BSSID: {}) {} olarak yapılandırıldı.", service_name, service.bssid, if is_active { "aktif" } else { "pasif" }));
        } else {
            println!("Hizmet {} bulunamadı.", service_name);
        }
    }
}
