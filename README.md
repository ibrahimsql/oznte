## OznDefense Network System
OznDefense, ağ güvenliği ve savunması üzerine odaklanmış bir araçtır. Bu sistem, ağ trafiğini izlemek ve çeşitli güvenlik tehditlerine karşı koruma sağlamak için tasarlanmıştır. 🛡️

#### Özellikler
Ağ Trafiği İzleme: Gerçek zamanlı ağ trafiğini izleme. 📈
Tehdit Algılama: Şüpheli faaliyetleri ve güvenlik tehditlerini algılama. 🚨
Raporlama: Güvenlik olayları ve ağ trafiği hakkında ayrıntılı raporlar sunma. 📊
## Kurulum
Gereksinimler
Rust programlama dili 🦀
cargo ve rustc araçları
Diğer bağımlılıklar için Cargo.toml dosyasına bakın
Adımlar
[Rust Resmi Sitesinden Rustı İndirin](https://www.rust-lang.org/)n.

### Depoyu Klonlayın:

git clone https://github.com/ibrahimsql/OznDefense-Network-System.git
Projeyi Derleyin:

cd OznDefense-Network-System
cargo build --release
### Uygulamayı Başlatın:


### ./target/release/ozn_defense
Kullanım
Temel Kullanım
OznDefense uygulamasını başlattıktan sonra, ağ trafiğini izlemeye başlamak için aşağıdaki adımları izleyebilirsiniz:

### Uygulamayı Başlatın:
./target/release/ozn_defense

### Ağ Trafiğini İzleme:

Uygulama başlatıldığında, otomatik olarak ağ trafiğini izlemeye başlayacaktır. Bu işlem sırasında kullanıcı arayüzünde gerçek zamanlı ağ trafiği ve güvenlik olayları hakkında bilgiler görüntülenecektir. 📡

Konfigürasyon Dosyasının Düzenlenmesi
config.toml dosyasını düzenleyerek ağ ayarlarınızı yapılandırabilirsiniz. Aşağıda örnek bir konfigürasyon dosyası bulunmaktadır:

toml
[network]
interface = "eth0"
log_level = "info"

[alerts]
enabled = true
threshold = 10
interface: İzlemek istediğiniz ağ arayüzünü belirtir.
log_level: Log seviyesini belirler (info, warn, error).
alerts: Uyarıları etkinleştirir ve eşik değerini ayarlar.
Örnek Kullanımlar

#### 1. El Sıkışma Yakalama (Handshake Capture) 🤝
Bu mod, bir Wi-Fi ağına bağlanan cihazların el sıkışmalarını yakalar. WPA/WPA2 şifrelerini kırmak için kullanılır.

cargo run -- --mode handshake_capture --interface wlan0 --target_bssid AA:BB:CC:DD:EE:FF --packet_count 100
--mode handshake_capture: El sıkışma yakalama modunu seçer.
--interface wlan0: Kullanılacak ağ arayüzü.
--target_bssid AA:BB:CC:DD:EE:FF: Hedefin BSSID'si.
--packet_count 100: Yakalamak için gönderilecek paket sayısı.


#### 2. Beacon Flooding 🌊
Bu mod, sahte erişim noktaları oluşturarak Wi-Fi tarayıcılarını yanıltır ve gerçek erişim noktalarını gizler.

cargo run -- --mode beacon_flood --interface wlan0 --packet_count 500 --target_ssid_prefix "FakeAP_"
--mode beacon_flood: Beacon flood saldırı modunu seçer.
--interface wlan0: Kullanılacak ağ arayüzü.
--packet_count 500: Gönderilecek beacon paketi sayısı.
--target_ssid_prefix "FakeAP_": Sahte SSID'lerin ön eki.


#### 3. Evil Twin Attack 👯‍♂️
Bu saldırı, bir kullanıcıyı sahte bir erişim noktasına bağlamaya çalışır.


cargo run -- --mode evil_twin --interface wlan0 --target_ssid "TargetSSID" --deauth_all true --log_path /path/to/log.txt
--mode evil_twin: Evil Twin saldırı modunu seçer.
--interface wlan0: Kullanılacak ağ arayüzü.
--target_ssid "TargetSSID": Hedef SSID adı.
--deauth_all true: Tüm istemcileri hedef alır.
--log_path /path/to/log.txt: Kayıt dosyasının yolu.

#### 4. PMKID Attack 🛠️
PMKID, WPA/WPA2 ağlarında bir tür el sıkışma yakalama yöntemidir. Bu komut PMKID'leri yakalamaya yöneliktir.

cargo run -- --mode pmkid_attack --interface wlan0 --target_bssid AA:BB:CC:DD:EE:FF --verbosity 3
--mode pmkid_attack: PMKID saldırı modunu seçer.
--interface wlan0: Kullanılacak ağ arayüzü.
--target_bssid AA:BB:CC:DD:EE:FF: Hedef BSSID.
--verbosity 3: Çıktının detay seviyesini belirler.

#### 5. Bruteforce WPA/WPA2 Attack 🔑
Bu saldırı, WPA/WPA2 şifrelerini brute-force yöntemiyle kırmaya çalışır.

cargo run -- --mode wpa_crack --interface wlan0 --dictionary_path /path/to/dictionary.txt --target_bssid AA:BB:CC:DD:EE:FF --verbosity 2
--mode wpa_crack: WPA/WPA2 şifre kırma modunu seçer.
--interface wlan0: Kullanılacak ağ arayüzü.
--dictionary_path /path/to/dictionary.txt: Sözlük dosyasının yolu.
--target_bssid AA:BB:CC:DD:EE:FF: Hedef BSSID.
--verbosity 2: Çıktının detay seviyesini belirler.

#### 6. Deauthentication Attack for All Clients ⚡
Bu saldırı, belirli bir erişim noktasına bağlı tüm cihazları hedef alır ve bağlantılarını kesmeye çalışır.

cargo run -- --mode deauth --interface wlan0 --target_bssid AA:BB:CC:DD:EE:FF --deauth_all true --packet_count 5000
--mode deauth: Deauthentication saldırı modunu seçer.
--interface wlan0: Kullanılacak ağ arayüzü.
--target_bssid AA:BB:CC:DD:EE:FF: Hedef BSSID.
--deauth_all true: Tüm istemcileri hedef alır.
--packet_count 5000: Gönderilecek paket sayısı.
Katkıda Bulunma
Katkıda bulunmak istiyorsanız, lütfen aşağıdaki adımları takip edin:

## [Fork](https://github.com/ibrahimsql/OznDefense-Network-System/)depoyu. 🍴
Yeni bir dal oluşturun (git checkout -b feature-branch).
Değişikliklerinizi yapın ve test edin. 🧪
Değişikliklerinizi commit edin ve push yapın (git push origin feature-branch). 🚀
[Pull Request Oluşturun](https://github.com/ibrahimsql/OznDefense-Network-System/pulls). 📥
[LICENSE](LICENSE)
Bu proje MIT Lisansı altında lisanslanmıştır. 📜

## İletişim
Daha fazla bilgi veya destek için [Benimle](https://www.instagram.com/ibrahimsql/)iletişime geçebilirsiniz. 📧

