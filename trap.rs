use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum TrapType {
    SSHHoneypot,
    HTTPHoneypot,
    FTPHoneypot,
    SMTPHoneypot,
    TelnetHoneypot,
    DNSHoneypot,
    SNMPHoneypot,
}

#[derive(Debug)]
pub struct Trap {
    pub name: String,
    pub trap_type: TrapType,
    pub active: bool,
    pub effectiveness: u32,
    pub duration: Option<u64>, // Tuzak süresi
    pub activation_time: Option<Instant>, // Aktif olduğu zaman
    pub comments: Vec<String>, // Kullanıcı yorumları
}

pub struct TrapManager {
    traps: HashMap<String, Trap>,
}

impl TrapManager {
    pub fn new() -> Self {
        Self {
            traps: HashMap::new(),
        }
    }

    pub fn add_trap(&mut self, name: &str, trap_type: TrapType, duration: Option<u64>) {
        let trap = Trap {
            name: name.to_string(),
            trap_type,
            active: true,
            effectiveness: 0,
            duration,
            activation_time: None,
            comments: Vec::new(),
        };
        self.traps.insert(name.to_string(), trap);
        println!("{} trap added.", name);
    }

    pub fn toggle_trap(&mut self, trap_name: &str) {
        if let Some(trap) = self.traps.get_mut(trap_name) {
            trap.active = !trap.active;
            println!("Trap {} status: {}.", trap_name, if trap.active { "Active" } else { "Inactive" });
            if trap.active {
                trap.activation_time = Some(Instant::now());
            } else {
                trap.activation_time = None;
            }
        } else {
            println!("Trap {} not found.", trap_name);
        }
    }

    pub fn simulate_trap_effect(&self, trap_name: &str) {
        if let Some(trap) = self.traps.get(trap_name) {
            if trap.active {
                println!("Simulating effect of trap: {}", trap.name);
                let trap_name_clone = trap.name.clone();
                thread::spawn(move || {
                    for _ in 0..10 {
                        println!("Trap {} activated by attacker!", trap_name_clone);
                        thread::sleep(Duration::from_secs(3));
                    }
                });
            } else {
                println!("Trap {} is not active.", trap_name);
            }
        }
    }

    pub fn update_trap_effectiveness(&mut self, trap_name: &str, effectiveness: u32) {
        if let Some(trap) = self.traps.get_mut(trap_name) {
            trap.effectiveness += effectiveness; // Cumulative effectiveness
            println!("Updated effectiveness of {} trap to {}.", trap_name, trap.effectiveness);
        } else {
            println!("Trap {} not found.", trap_name);
        }
    }

    pub fn report_trap_statistics(&self) {
        println!("Trap Statistics Report:");
        for trap in self.traps.values() {
            let active_time = if let Some(start) = trap.activation_time {
                start.elapsed().as_secs()
            } else {
                0
            };
            println!(
                "Trap: {}, Type: {:?}, Status: {}, Effectiveness: {}, Active Time: {} seconds, Comments: {:?}",
                trap.name,
                trap.trap_type,
                if trap.active { "Active" } else { "Inactive" },
                trap.effectiveness,
                active_time,
                trap.comments
            );
        }
    }

    pub fn deploy_with_duration(&mut self, trap_name: &str) {
        if let Some(trap) = self.traps.get_mut(trap_name) {
            if trap.active {
                let duration = trap.duration.unwrap_or(0);
                println!("Deploying {} trap for {} seconds...", trap.name, duration);
                let trap_name_clone = trap.name.clone();
                thread::spawn(move || {
                    thread::sleep(Duration::from_secs(duration));
                    println!("{} trap deactivated after {} seconds.", trap_name_clone, duration);
                });
            } else {
                println!("Trap {} is not active.", trap_name);
            }
        }
    }

    pub fn reset_trap(&mut self, trap_name: &str) {
        if let Some(trap) = self.traps.get_mut(trap_name) {
            trap.effectiveness = 0;
            trap.activation_time = None;
            trap.comments.clear(); // Clear comments on reset
            println!("Trap {} has been reset.", trap_name);
        } else {
            println!("Trap {} not found.", trap_name);
        }
    }

    pub fn list_all_traps(&self) {
        println!("Listing all traps:");
        for trap in self.traps.values() {
            println!(
                "Name: {}, Type: {:?}, Active: {}, Effectiveness: {}",
                trap.name, trap.trap_type, trap.active, trap.effectiveness
            );
        }
    }

    pub fn remove_trap(&mut self, trap_name: &str) {
        if self.traps.remove(trap_name).is_some() {
            println!("Trap {} removed.", trap_name);
        } else {
            println!("Trap {} not found.", trap_name);
        }
    }

    pub fn add_comment(&mut self, trap_name: &str, comment: &str) {
        if let Some(trap) = self.traps.get_mut(trap_name) {
            trap.comments.push(comment.to_string());
            println!("Comment added to {} trap: {}", trap_name, comment);
        } else {
            println!("Trap {} not found.", trap_name);
        }
    }

    pub fn evaluate_trap_performance(&self) {
        println!("Evaluating trap performance...");
        for trap in self.traps.values() {
            let performance_score = trap.effectiveness / (trap.comments.len() as u32 + 1);
            println!("Performance of {}: {}", trap.name, performance_score);
        }
    }
}

// Ana fonksiyonu genişletme
fn main() {
    let mut trap_manager = TrapManager::new();

    // Tuzak türlerini tanımlayın ve ekleyin
    trap_manager.add_trap("SSH Honeypot 1", TrapType::SSHHoneypot, Some(15));
    trap_manager.add_trap("HTTP Honeypot 1", TrapType::HTTPHoneypot, Some(10));
    trap_manager.add_trap("FTP Honeypot 1", TrapType::FTPHoneypot, Some(12));
    trap_manager.add_trap("SMTP Honeypot 1", TrapType::SMTPHoneypot, Some(8));
    trap_manager.add_trap("Telnet Honeypot 1", TrapType::TelnetHoneypot, Some(20));
    trap_manager.add_trap("DNS Honeypot 1", TrapType::DNSHoneypot, Some(5));
    trap_manager.add_trap("SNMP Honeypot 1", TrapType::SNMPHoneypot, Some(7));

    // Tuzak simülasyonunu başlat
    trap_manager.simulate_trap_effect("SSH Honeypot 1");
    trap_manager.report_trap_statistics();

    // Tuzak durumu değiştirin ve yeniden raporlayın
    trap_manager.toggle_trap("HTTP Honeypot 1");
    trap_manager.report_trap_statistics();

    // Tuzakları belirli bir süreyle devreye alın
    trap_manager.deploy_with_duration("HTTP Honeypot 1");

    // Ek tuzaklar ve simülasyonlar
    for i in 2..=5 {
        trap_manager.add_trap(&format!("SSH Honeypot {}", i), TrapType::SSHHoneypot, Some(15));
        trap_manager.add_trap(&format!("HTTP Honeypot {}", i), TrapType::HTTPHoneypot, Some(10));
        trap_manager.add_trap(&format!("FTP Honeypot {}", i), TrapType::FTPHoneypot, Some(12));
        trap_manager.add_trap(&format!("SMTP Honeypot {}", i), TrapType::SMTPHoneypot, Some(8));
        trap_manager.add_trap(&format!("Telnet Honeypot {}", i), TrapType::TelnetHoneypot, Some(20));
        trap_manager.add_trap(&format!("DNS Honeypot {}", i), TrapType::DNSHoneypot, Some(5));
        trap_manager.add_trap(&format!("SNMP Honeypot {}", i), TrapType::SNMPHoneypot, Some(7));
    }

    // Tüm tuzakları rap
// Tüm tuzakları raporla
    trap_manager.report_trap_statistics();

    // Tuzaklar için yorum ekleyin
    trap_manager.add_comment("SSH Honeypot 1", "Very effective trap!");
    trap_manager.add_comment("HTTP Honeypot 1", "Caught several attackers.");
    trap_manager.add_comment("FTP Honeypot 1", "Not very effective.");
    trap_manager.add_comment("SMTP Honeypot 1", "Works well, but needs adjustments.");

    // Tuzak performansını değerlendir
    trap_manager.evaluate_trap_performance();

    // Tüm tuzakları listele
    trap_manager.list_all_traps();

    // Belirli bir tuzağı kaldır
    trap_manager.remove_trap("HTTP Honeypot 1");

    // Son durumu raporla
    trap_manager.report_trap_statistics();
}
