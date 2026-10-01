PROYECTO DE INVESTIGACIÓN: Generación de Perfiles Sintéticos para Análisis de Consistencia de Fingerprinting

CONTEXTO:
Desarrollo de biblioteca Rust para generación de perfiles de navegador sintéticos 
determinísticos. El objetivo es crear identidades coherentes (atributos internamente 
consistentes) para estudios de correlación en sistemas de fingerprinting.

REQUISITOS TÉCNICOS:

Biblioteca Rust crate `sgida-identity` con API pública limpia.

STACK RUST:
- RNG: rand + rand_chacha (ChaCha12 - determinístico, criptográficamente seguro)
- Serialización: serde + serde_json
- Validación: validator (derive macros)
- Time: chrono (timezones)
- Crypto: ring o rust-crypto (hashing)
- DB: rusqlite (perfiles almacenados localmente)

GENERACIÓN DE PERFILES:

Cada Profile debe contener:

pub struct Profile {
    pub id: Uuid,
    pub seed: [u8; 32], // Seed para reproducibilidad
    pub browser: BrowserProfile,
    pub system: SystemProfile,
    pub network: NetworkProfile,
    pub behavior: BehaviorProfile,
    pub credentials: Credentials,
}

pub struct BrowserProfile {
    pub user_agent: String, // Firefox/Chrome/Safari realista
    pub viewport: (u32, u32),
    pub color_depth: u8,
    pub pixel_ratio: f32,
    pub languages: Vec<String>, // Accept-Language
    pub platform: String, // Win32, MacIntel, Linux x86_64
    pub oscpu: Option<String>,
    pub product_sub: String, // 20100101, 20030107, etc.
    pub vendor: String,
}

pub struct SystemProfile {
    pub hardware_concurrency: u8, // 2-16, coherente con platform
    pub device_memory: f32, // GB, coherente con concurrency
    pub timezone: String, // Europe/Madrid, America/New_York
    pub locale: String, // es-ES, en-US
    pub screen_resolution: (u32, u32),
    pub available_screen: (u32, u32), // Menor que screen
    pub color_gamut: String,
}

REGLAS DE COHERENCIA (CRÍTICO):

Implementar validador que garantice:

1. Si platform es "Win32", user_agent debe contener "Windows"
2. Si platform es "MacIntel", timezone debe ser coherente con locale (ej: en-US rara vez con Asia/Tokyo)
3. hardware_concurrency debe ser potencia de 2 (2, 4, 8, 16) para browsers modernos
4. device_memory debe ser 0.25, 0.5, 1, 2, 4, 8 (estándar JS)
5. viewport < screen_resolution
6. timezone offset debe coincidir con timezone seleccionado (usar chrono-tz)

GENERACIÓN DETERMINÍSTICA:

impl Profile {
    pub fn from_seed(seed: [u8; 32]) -> Self {
        // Usar ChaCha12Rng::from_seed(seed)
        // Generar todos los atributos desde el RNG
        // Garantizar misma salida para misma entrada (reproducible)
    }
    
    pub fn random() -> Self {
        // Generar seed aleatorio con OsRng
        Self::from_seed(seed)
    }
}

CANVAS/WEBGL FINGERPRINT:

pub struct CanvasProfile {
    pub noise_seed: u32, // Para randomización consistente por sesión
    pub webgl_vendor: String,
    pub webgl_renderer: String,
}

Implementar función que genere webgl_vendor/renderer coherente con el sistema operativo:
- Windows: "Google Inc. (NVIDIA)" / "ANGLE (NVIDIA, NVIDIA GeForce GTX 1660..."
- macOS: "Apple Inc." / "Apple M1"
- Linux: variado

CREDENCIALES:

pub struct Credentials {
    pub username: String, // Generado con petname o similar
    pub password: String, // Generado con diceware o passphrase
    pub email_local: String, // Para derivar email
    pub birth_date: NaiveDate, // Coherente con edad
}

Generación de usernames realistas (no random strings):
- Usar crate `petname` (adjetivo-sustantivo-número: "blue-river-42")
- O generador de nombres basado en locale (faker-rs)

VALIDACIÓN:

Implementar trait Validator:
- fn validate_coherence(&self) -> Result<(), ValidationError>
- Debe checkear todas las reglas de coherencia mencionadas

SERIALIZACIÓN:

- Serialize/Deserialize para JSON y TOML
- Guardar en SQLite con schema:
  CREATE TABLE profiles (
    id BLOB PRIMARY KEY,
    seed BLOB NOT NULL,
    data JSON NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
  );

ENTREGABLES:
1. Estructuras de datos completas con derives (Debug, Clone, Serialize, Deserialize)
2. Implementación de generación determinística desde seed
3. Validador de coherencia exhaustivo
4. Tests unitarios que verifiquen coherencia (proptest para fuzzing)
5. Ejemplos de uso en docs/ (cargo doc)
6. Benchmarks con criterion.rs para generación de 1000 perfiles

CONSIDERACIONES DE PRIVACIDAD:
- No hardcodear listas de UA (cargar desde archivo TOML/JSON)
- No incluir telemetría en la biblioteca
- Zero network calls en esta crate