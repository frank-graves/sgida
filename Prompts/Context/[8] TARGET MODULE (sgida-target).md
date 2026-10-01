PROYECTO DE INVESTIGACIÓN: Framework de Automatización de Interacción Web para Pruebas de Seguridad

CONTEXTO:
Desarrollo de framework declarativo para automatización de interacciones web 
complejas en entornos de prueba controlados. El objetivo es validar flujos 
de usuario y probar la robustez de sistemas de autenticación propios.

REQUISITOS TÉCNICOS:

Crate `sgida-target` que proporciona abstracciones sobre Playwright.

STACK RUST:
- Playwright: playwright-rs (bindings oficiales o community)
- Async: tokio
- Parsing HTML: scraper (selectores CSS)
- URL: url crate
- Serialization: serde

ARQUITECTURA:

1. Browser Controller Wrapper:
   - Abstracción sobre playwright::Page
   - Integración con Stealth Module (inyección de scripts)
   - Configuración de viewport, UA, etc. desde Profile

pub struct ControlledBrowser {
    browser: playwright::Browser,
    context: playwright::BrowserContext,
    page: playwright::Page,
    profile: Profile,
}

impl ControlledBrowser {
    pub async fn new(profile: &Profile, stealth: &StealthConfig) -> Result<Self, Error>;
    pub async fn navigate(&self, url: &str) -> Result<(), Error>;
    pub async fn find_element(&self, selector: &str) -> Result<Element, Error>;
    pub async fn click_human(&self, element: &Element) -> Result<(), Error>; // Usa Behavior Module
    pub async fn type_human(&self, element: &Element, text: &str) -> Result<(), Error>;
}

2. Site Adapters (Plugin System):
   - Trait SiteAdapter que define interacción con un sitio específico
   - Implementaciones para flujos comunes: registro, login, etc.

pub trait SiteAdapter {
    async fn register(&self, browser: &ControlledBrowser, account: &Account) -> Result<RegistrationResult, Error>;
    async fn login(&self, browser: &ControlledBrowser, credentials: &Credentials) -> Result<Session, Error>;
    async fn is_logged_in(&self, browser: &ControlledBrowser) -> Result<bool, Error>;
}

3. Form Handlers:
   - Detección automática de campos de formulario (input[type="email"], etc.)
   - Mapeo de datos a campos
   - Manejo de validación cliente (JavaScript)

pub struct Form {
    fields: Vec<FormField>,
}

impl Form {
    pub async fn detect(page: &playwright::Page) -> Result<Self, Error>;
    pub async fn fill(&self, data: &HashMap<String, String>) -> Result<(), Error>;
    pub async fn submit(&self) -> Result<(), Error>;
}

4. CAPTCHA Detection (Solo detección, no resolución):
   - Detectar presencia de reCAPTCHA, hCaptcha, Cloudflare
   - Reportar al orquestador para manejo externo
   - NO implementar solución automática (separar responsabilidades)

pub enum CaptchaType {
    ReCaptchaV2,
    ReCaptchaV3,
    HCaptcha,
    CloudflareChallenge,
    Unknown,
}

pub fn detect_captcha(page: &playwright::Page) -> Result<Option<CaptchaType>, Error>;

5. Recovery Manager:
   - Manejo de errores: timeout, element not found, navigation failed
   - Estrategias de retry con backoff
   - Screenshots en fallos para debugging

pub struct RecoveryManager {
    max_retries: u32,
    backoff: ExponentialBackoff,
}

ERROR HANDLING:

Usar thiserror para errores específicos:

#[derive(Error, Debug)]
pub enum TargetError {
    #[error("element not found: {selector}")]
    ElementNotFound { selector: String },
    #[error("navigation failed: {url}")]
    NavigationFailed { url: String },
    #[error("captcha detected: {captcha_type:?}")]
    CaptchaDetected { captcha_type: CaptchaType },
    #[error("timeout after {duration:?}")]
    Timeout { duration: Duration },
}

INTEGRACIÓN CON OTROS MÓDULOS:

- Recibe Profile de Identity Module
- Usa StealthModule para configuración anti-detection
- Usa BehaviorModule para interacciones humanizadas
- Reporta a Core Orchestrator vía API/events

ENTREGABLES:
1. Wrapper de Playwright con integración de stealth
2. Sistema de Site Adapters extensible
3. Detector de formularios automático
4. Recovery Manager con retry logic
5. Ejemplos de adapters para flujos comunes (registro genérico)
6. Tests de integración con Playwright

CONSIDERACIONES:
- Playwright requiere descargar browsers (hacerlo en build.rs o documentar)
- Manejar correctamente el lifecycle de browser (siempre cerrar al final)
- No hardcodear selectores CSS (cargar desde configuración)
- Logging de acciones (no de datos sensibles)