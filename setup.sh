#!/bin/bash

# Betik hata durumunda hemen duracak
set -e

# Log dosyası için bir isim belirleyelim
LOGFILE="setup.log"
ERRORFILE="setup_errors.log"

# Log dosyalarını sıfırla veya oluştur
echo "Kurulum işlemleri başlatılıyor..." > "$LOGFILE"
echo "Kurulum hataları (varsa)..." > "$ERRORFILE"

# Funksiyon: Log'a yazma
log() {
    echo "$1" | tee -a "$LOGFILE"
}

# Funksiyon: Hata yönetimi
error_exit() {
    log "$1" >&2
    echo "$1" >> "$ERRORFILE"
    exit 1
}

# Sistem gereksinimlerini kontrol et
log "Sistem gereksinimlerini kontrol etme..."
REQUIRED_COMMANDS=("cargo" "curl" "git")

for cmd in "${REQUIRED_COMMANDS[@]}"; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
        error_exit "$cmd bulunamadı. Bu betiği çalıştırmak için $cmd'yi yüklemeniz gerekiyor."
    fi
done

log "Gerekli komutlar kurulu."

# Rust ve Cargo'nun yüklü olduğundan emin olun
if ! command -v cargo >/dev/null 2>&1; then
    log "Cargo bulunamadı. Rust'ı yüklemeniz gerekiyor."
    log "Rust'ı yüklemek için lütfen https://www.rust-lang.org/tools/install adresini ziyaret edin."
    error_exit "Kurulum tamamlanamadı."
fi

log "Cargo bulunuyor."

# Kullanıcıdan onay alma
read -p "Devam etmek istiyor musunuz? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    log "Kurulum iptal edildi."
    exit 0
fi

log "Devam ediliyor..."

# Depoyu klonla (eğer gerekiyorsa)
REPO_URL="https://github.com/ibrahimsql/OznDefense-Network-System.git"
if [ ! -d "OznDefense-Network-System" ]; then
    log "Depoyu klonlama..."
    git clone "$REPO_URL" >> "$LOGFILE" 2>> "$ERRORFILE" || error_exit "Depoyu klonlama başarısız."
fi

cd OznDefense-Network-System

# Projeyi güncelle
log "Projeyi güncelleme ve bağımlılıkları yükleme..."
cargo update >> "$LOGFILE" 2>> "$ERRORFILE" || error_exit "Projeyi güncelleme başarısız."

# Projeyi derle
log "Projeyi derleme..."
cargo build --release >> "$LOGFILE" 2>> "$ERRORFILE" || error_exit "Projeyi derleme başarısız."

# Testleri çalıştır
log "Testleri çalıştırma..."
cargo test >> "$LOGFILE" 2>> "$ERRORFILE" || error_exit "Testler başarısız."

# Temizlik işlemleri
log "Temizlik işlemleri yapılıyor..."
cargo clean >> "$LOGFILE" 2>> "$ERRORFILE" || error_exit "Temizlik işlemleri başarısız."

# Kullanıcıya sonuçları bildirme
log "Kurulum tamamlandı ve testler başarılı!"

# Log dosyasının konumunu bildirme
log "Kurulum log dosyası: $LOGFILE"
log "Kurulum hataları (varsa): $ERRORFILE"

echo "Kurulum işlemi tamamlandı. Lütfen log dosyalarını kontrol edin."
